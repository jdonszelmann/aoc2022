def main():
    inp = [i.split() for i in open("data.in").read().split("\n")]
    for snek_length in [1, 9]:
        head = (0, 0)
        snek = [(0, 0) for _ in range(snek_length)]

        around = [(0, 0), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]

        def add(a, b):
            return a[0] + b[0], a[1] + b[1]

        def dist(a, b):
            return abs(a[0] - b[0]), abs(a[1] - b[1])

        had = set()

        for (d, n) in inp:
            dir = {"R": (0, 1), "L": (0, -1), "D": (-1, 0), "U": (1, 0)}[d]
            for _ in range(int(n)):
                head = add(head, dir)
                prev_after_move = head

                for t in range(len(snek)):
                    if all(add(snek[t], a) != prev_after_move for a in around):
                        snek[t] = min((add(i, snek[t]) for i in around[1:]), key=lambda i: dist(i, prev_after_move))

                    prev_after_move = snek[t]

                had.add(snek[-1])

        print(len(had))


if __name__ == '__main__':
    main()
