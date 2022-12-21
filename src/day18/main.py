import functools
import heapq
from collections import deque

from tqdm import tqdm


def add(a, b):
    return a[0] + b[0], a[1] + b[1], a[2] + b[2]



def main():
    inp = [i for i in open("data.in").read().split("\n")]

    points = set()
    for i in inp:
        x, y, z = i.split(",")
        points.add((int(x), int(y), int(z)))

    total = 0
    for i in points:
        for j in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)]:
            if add(i, j) not in points:
                total += 1

    @functools.cache
    def pathfind(a, b):
        if a in points:
            return False

        print(f"from {a} to {b}")
        todo = deque()
        todo.append(a)

        had = set()

        while todo:
            next = todo.popleft()
            if next in had:
                continue
            had.add(next)

            if next == b:
                print("ok")
                return True

            for i in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)]:
                res = add(next, i)
                if res not in had and res not in points:
                    todo.append(res)

        print("no path")
        return False

    total = 0
    for i in tqdm(points):
        for j in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)]:
            if pathfind(add(i, j), (20, 20, 20)):
                total += 1

    print(total)

if __name__ == '__main__':
    main()  
                