
def add_coord(a, b):
    return a[0] + b[0], a[1] + b[1]


def main():
    inp = [[[int(a) for a in j.strip().split(",")] for j in i.split("->")] for i in open("data.in").read().split("\n")]

    cave = set()
    sand = set()
    floor = 0

    def draw_cave():
        for y in range(0, 12):
            for x in range(500-12, 500+13):
                if (x, y) in cave:
                    print("#", end="")
                elif y == floor:
                    print("#", end="")
                elif (x, y) in sand:
                    print("o", end="")
                else:
                    print(".", end="")
            print()

    # generate cave
    for i in inp:
        src = i[0]
        for dst in i[1:]:
            dx = (dst[0] - src[0])
            dy = (dst[1] - src[1])

            dx = 0 if dx == 0 else -1 if dx < 0 else 1
            dy = 0 if dy == 0 else -1 if dy < 0 else 1

            if dx == 0:
                for y in range(src[1], dst[1], dy):
                    cave.add((src[0], y))
            if dy == 0:
                for x in range(src[0], dst[0], dx):
                    cave.add((x, src[1]))

            cave.add((dst[0], dst[1]))
            src = dst


    for i in inp:
        for pt in i:
            floor = max(pt[1], floor)

    floor += 2

    def drop_sand_grain():
        pos = (500, 0)
        down = (0, 1)
        left = (-1, 1)
        right = (1, 1)

        def blocked(x):
            return x in cave or x in sand

        while True:
            step_down = add_coord(pos, down)
            step_left = add_coord(pos, left)
            step_right = add_coord(pos, right)


            if not blocked(step_down):
                pos = step_down
            elif not blocked(step_left):
                pos = step_left
            elif not blocked(step_right):
                pos = step_right
            else:
                sand.add(pos)
                return True

            if pos[1] > 200:
                return False

    def drop_sand_grain_2():
        pos = (500, 0)
        down = (0, 1)
        left = (-1, 1)
        right = (1, 1)

        if pos in sand:
            return False

        def blocked(x):
            return x in cave or x in sand or x[1] == floor

        while True:
            step_down = add_coord(pos, down)
            step_left = add_coord(pos, left)
            step_right = add_coord(pos, right)

            if not blocked(step_down):
                pos = step_down
            elif not blocked(step_left):
                pos = step_left
            elif not blocked(step_right):
                pos = step_right
            else:
                sand.add(pos)
                return True

    while drop_sand_grain():
        pass
    print(len(sand))

    sand = set()

    while drop_sand_grain_2():
        pass
    print(len(sand))

if __name__ == '__main__':
    main()  
                