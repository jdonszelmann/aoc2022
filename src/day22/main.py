
import re


def add(a, b):
    return a[0] + b[0], a[1] + b[1]

def scale(a, b):
    return a[0] * b, a[1] * b

def main():
    board_map, route = [i for i in open("data.in").read().split("\n\n")]

    walls = {}
    minx, miny, maxx, maxy = 10000, 10000, 0, 0
    for y, row in enumerate(board_map.split("\n")):
        for x, col in enumerate(row):
            if col != " ":
                minx = min(minx, x)
                maxx = max(maxx, x)
                miny = min(miny, y)
                maxy = max(maxy, y)

            if col == ".":
                walls[(x, y)] = False
            elif col == "#":
                walls[(x, y)] = True

    leftmost = {}
    rightmost = {}
    topmost = {}
    bottommost = {}


    for y in range(miny, maxy + 1):
        for x in range(minx, maxx + 1):
            if (x, y) in walls:
                leftmost[y] = x
                break

    for y in range(miny, maxy + 1):
        for x in range(maxx + 1, minx, -1):
            if (x, y) in walls:
                rightmost[y] = x
                break

    for x in range(minx, maxx + 1):
        for y in range(miny, maxy + 1):
            if (x, y) in walls:
                topmost[x] = y
                break

    for x in range(minx, maxx + 1):
        for y in range(maxy + 1, miny, -1):
            if (x, y) in walls:
                bottommost[x] = y
                break

    start = (leftmost[0], 0)
    assert start is not None

    direction = (1, 0)
    pos = start
    path = {start: direction}

    def print_path():
        for y in range(miny, maxy+1):
            for x in range(minx, maxx + 1):
                if (x, y) in walls and walls[(x, y)]:
                    print("#", end="")
                elif (x, y) in walls and not walls[(x, y)]:
                    if (x, y) in path:
                        match path[(x, y)]:
                            case (0, 1):
                                print("v", end="")
                            case (0, -1):
                                print("^", end="")
                            case (1, 0):
                                print(">", end="")
                            case (-1, 0):
                                print("<", end="")
                            case _:
                                raise Exception("unreachable")
                    else:
                        print(".", end="")
                else:
                    print(" ", end="")

            print()

        print("----")

    for i in re.split(r"([RL])", route):
        # print_path()
        if i == "R":
            direction = (-direction[1], direction[0])
        elif i == "L":
            direction = (direction[1], -direction[0])
        else:
            for _ in range(int(i)):
                newpos = list(add(pos, direction))

                if direction == (1, 0) and newpos[0] > rightmost[newpos[1]]:
                    newpos[0] = leftmost[newpos[1]]
                if direction == (-1, 0) and newpos[0] < leftmost[newpos[1]]:
                    newpos[0] = rightmost[newpos[1]]

                if direction == (0, -1) and newpos[1] < topmost[newpos[0]]:
                    newpos[1] = bottommost[newpos[0]]
                if direction == (0, 1) and newpos[1] > bottommost[newpos[0]]:
                    newpos[1] = topmost[newpos[0]]

                if walls[tuple(newpos)]:
                    break
                else:
                    path[tuple(newpos)] = direction
                    pos = tuple(newpos)

    row = pos[1] + 1
    col = pos[0] + 1
    match direction:
        case (0, 1): facing = 1
        case (0, -1): facing = 3
        case (1, 0): facing = 0
        case (-1, 0): facing = 2
        case _: raise Exception("unreachable")

    print(row, col, facing)
    print(1000 * row + col * 4 + facing)




if __name__ == '__main__':
    main()