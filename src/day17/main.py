import copy
from itertools import cycle
from collections import deque, defaultdict

from tqdm import tqdm


def main():
    inp = [i for i in open("data.in").read()]

    shapes = {
        "-": {
            "height": 1,
            "parts": [(0, 0), (1, 0), (2, 0), (3, 0)]
        },
        "+": {
            "height": 3,
            "parts": [(0, 1), (1, 1), (2, 1), (1, 0), (1, 2)]
        },
        "L": {
            "height": 3,
            "parts": [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]
        },
        "|": {
            "height": 4,
            "parts": [(0, 0), (0, 1), (0, 2), (0, 3)]
        },
        "#": {
            "height": 2,
            "parts": [(0, 0), (0, 1), (1, 0), (1, 1)]
        },
    }

    def print_room():
        for row in range(height):
            print("|", end="")
            for i in range(7):
                print(room[height - row - 1][1][i], end="")
            print("|")
        print("+-------+", end="")

    def draw_shape(x, y, shape):
        for i in shapes[shape]["parts"]:
            room[y + i[1]][1][x + i[0]] = "#"

    def try_draw_shape(x, y, shape):
        if y == -1:
            return False

        for i in shapes[shape]["parts"]:
            if x + i[0] < 0 or x + i[0] >= 7:
                return False

            if room[y + i[1]][1][x + i[0]] == "#":
                return False
        return True

    def remove_shape(x, y, shape):
        for i in shapes[shape]["parts"]:
            room[y + i[1]][1][x + i[0]] = " "

    hyperperiod = len(inp) * 5
    empty_row = [' ', ' ', ' ', ' ', ' ', ' ', ' ']
    room = deque()
    largest_y = 0
    height = 0
    shape = cycle(["-", "+", "L", "|", "#"])
    direction = cycle(inp)
    removed = 0

    def try_reduce_room():
        nonlocal largest_y
        nonlocal height
        nonlocal removed

        can_remove = 0
        for idx, i in enumerate(room):
            if i[1] == ["#", "#", "#", "#", "#", "#", "#"]:
                can_remove = idx - 1

        for i in range(can_remove):
            room.popleft()

        largest_y -= can_remove
        height -= can_remove
        removed += can_remove


    part1 = 2022
    part2 = 1000000000000
    curr = part2
    topshapes = {}
    skip = False
    itercnt = 0

    while itercnt < curr:
        itercnt += 1
        # try_reduce_room()
        if itercnt % hyperperiod == 0 and not skip:
            topshape = []
            for x in range(7):
                ctr = 0
                for y in range(height-1, 0, -1):
                    if room[y][1][x] == "#":
                        break
                    ctr += 1

                topshape.append(ctr)
            topshape = tuple(topshape)
            print(topshape)
            if topshape in topshapes:
                print(f"already found!: at {topshapes[topshape][0]} the topshape was also {topshape}, now: {(largest_y + removed)}")
                rocks_to_go = curr - itercnt
                period = itercnt - topshapes[topshape][1]
                growth = (largest_y + removed) - topshapes[topshape][0]
                print(f"every period: {period}, the stack will grow by {growth} from now on")
                print(f"{rocks_to_go} more rocks to go")
                print(f"{rocks_to_go // period} more periods to go")
                more_height = (rocks_to_go // period) * growth
                print(f"{more_height} more height to go")
                final_y = more_height + (largest_y + removed)
                print(f"{final_y} final height")

                skipped = (rocks_to_go // period) * period
                leftover = rocks_to_go - skipped
                print(f"{skipped} cycles skipped, {leftover} cycles not accounted for")
                itercnt += skipped
                removed += more_height
                skip = True

            topshapes[topshape] = (largest_y + removed, itercnt)


        curr_shape = next(shape)
        new_y = largest_y + 3
        shape_height = shapes[curr_shape]["height"]
        for i in range(height, new_y + shape_height):
            room.append((i, copy.deepcopy(empty_row)))
            height += 1

        x = 2
        y = new_y
        first = True

        while True:
            if first:
                # print("a new rock begins falling")
                draw_shape(x, y, curr_shape)
                # print_room()
                # input()
                first = False
            else:
                remove_shape(x, y, curr_shape)
                if try_draw_shape(x, y - 1, curr_shape):
                    # print("falls 1 unit")
                    y -= 1
                    draw_shape(x, y, curr_shape)
                else:
                    # print("falls 1 unit causing it to rest")
                    draw_shape(x, y, curr_shape)
                    largest_y = max(y + shapes[curr_shape]["height"], largest_y)
                    break
            # print_room()
            # input()

            dir = 1 if next(direction) == ">" else -1
            if dir == 1:
                # print("jet of gas pushes to the right")
                pass
            remove_shape(x, y, curr_shape)
            if try_draw_shape(x + dir, y, curr_shape):
                x += dir
            else:
                # print("but nothing happens")
                pass
            draw_shape(x, y, curr_shape)

            # print_room()
            # input()

    print(largest_y + removed)

if __name__ == '__main__':
    main()