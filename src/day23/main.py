from collections import defaultdict


def main():
    inp = [i for i in open("data.in").read().split("\n")]

    elves = set()
    for y, row in enumerate(inp):
        for x, col in enumerate(row):
            if col == "#":
                elves.add((x, y))
    # print(elves)

    def print_elves():
        minx, maxx, miny, maxy = 10000, -10000, 10000, -1000
        for (x, y) in elves:
            minx = min(minx, x)
            miny = min(miny, y)
            maxx = max(maxx, x)
            maxy = max(maxy, y)

        for y in range(miny - 1, maxy + 1):
            for x in range(minx - 1, maxx + 1):
                if (x, y) in elves:
                    print("#", end="")
                else:
                    print(".", end="")
            print()
        print("---")

    consider = ["N", "S", "W", "E"]

    i = 0
    while True:
        i += 1
        # print_elves()
        oldelves = elves
        new_elves = defaultdict(int)
        old_pos = defaultdict(list)
        oldlen = len(elves)

        for x, y in elves:
            if (

                    (x - 1, y - 1) not in elves and
                    (x, y - 1) not in elves and
                    (x + 1, y - 1) not in elves and

                    (x - 1, y + 1) not in elves and
                    (x, y + 1) not in elves and
                    (x + 1, y + 1) not in elves and

                    (x + 1, y) not in elves and
                    (x - 1, y) not in elves
            ):
                new_elves[(x, y)] += 1
                old_pos[(x, y)].append((x, y))
                # print("stay")
            else:
                for c in consider:
                    if (
                            c == "N" and
                            (x - 1, y - 1) not in elves and
                            (x, y - 1) not in elves and
                            (x + 1, y - 1) not in elves
                    ):
                        # print("north")
                        new_elves[(x, y - 1)] += 1
                        old_pos[(x, y - 1)].append((x, y))
                        break

                    if (
                            c == "S" and
                            (x - 1, y + 1) not in elves and
                            (x, y + 1) not in elves and
                            (x + 1, y + 1) not in elves
                    ):
                        # print("south")
                        new_elves[(x, y + 1)] += 1
                        old_pos[(x, y + 1)].append((x, y))
                        break

                    if (
                            c == "W" and
                            (x - 1, y - 1) not in elves and
                            (x - 1, y) not in elves and
                            (x - 1, y + 1) not in elves
                    ):
                        # print("west")
                        new_elves[(x - 1, y)] += 1
                        old_pos[(x - 1, y)].append((x, y))
                        break

                    if (
                            c == "E" and
                            (x + 1, y - 1) not in elves and
                            (x + 1, y) not in elves and
                            (x + 1, y + 1) not in elves
                    ):
                        # print("east")
                        new_elves[(x + 1, y)] += 1
                        old_pos[(x + 1, y)].append((x, y))
                        break

                else:
                    new_elves[(x, y)] += 1
                    old_pos[(x, y)].append((x, y))

        elves = set()
        # print("unique", len(new_elves))
        # print("sum", sum(i for i in new_elves.values()))
        for k, v in new_elves.items():
            if v == 1:
                elves.add(k)
            else:
                for p in old_pos[k]:
                    elves.add(p)

        consider.append(consider.pop(0))

        assert len(elves) == oldlen
        # print(elves)

        if oldelves == elves:
            print(f"NO MOVES IN ROUND {i}")
            break

    # print_elves()
    minx, maxx, miny, maxy = 10000, -10000, 10000, -1000
    for (x, y) in elves:
        minx = min(minx, x)
        miny = min(miny, y)
        maxx = max(maxx, x)
        maxy = max(maxy, y)
    width = (maxx + 1) - minx
    height = (maxy + 1) - miny
    area = width * height
    print(width, height, area, area - len(elves))



if __name__ == '__main__':
    main()
