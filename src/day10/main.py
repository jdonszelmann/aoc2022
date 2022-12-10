
def main():
    inp = [i for i in open("data.in").read().split("\n")]

    x = 1
    c = 0

    render = []

    def inc_cycle():
        nonlocal c
        c += 1

        if (c - 20) % 40 == 0:
            render.append(c * x)

    for i in inp:
        match i.split():
            case ["noop"]:
                inc_cycle()
            case ["addx", n]:
                inc_cycle()
                inc_cycle()
                x += int(n)

    print(sum(render))

    x = 1
    y = 0
    c = 0

    render = [["." for _ in range(40)] for _ in range(6)]

    def inc_cycle():
        nonlocal c, y, render
        beam_x = c % 40
        c += 1
        if beam_x in [x-1, x, x+1]:
            render[y][beam_x] = "#"

        if beam_x == 39:
            y += 1
            if y > 5:
                y = 0

    for i in inp:
        match i.split():
            case ["noop"]:
                inc_cycle()
            case ["addx", n]:
                inc_cycle()
                inc_cycle()
                x += int(n)

    for i in range(6):
        print("".join(render[i]))

if __name__ == '__main__':
    main()  
                