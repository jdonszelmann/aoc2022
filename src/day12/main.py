from __future__ import annotations

import dataclasses
import heapq

def add(a, b):
    return a[0] + b[0], a[1] + b[1]

@dataclasses.dataclass
class Node:
    curr: (int, int)
    parent: Node | None
    name: str
    cost: int
    dir: str

    def neighbors(self):
        return [(a, add(self.curr, b)) for (a, b) in [("^", (-1, 0)), ("v", (1, 0)), (">", (0, 1)), ("<", (0, -1))]]

    def __lt__(self, other):
        return self.cost < other.cost


def main():
    inp = [i for i in open("data.in").read().split("\n")]
    # start_y = 0
    # start_x = 0
    # for y in range(len(inp)):
    #     if "S" in inp[y]:
    #        start_y = y
    #        start_x = inp[y].index("S")
    #        break

    # print(start_y, start_x)

    todo = []
    for y in range(len(inp)):
        for x in range(len(inp[0])):
            if inp[y][x] in "aS":
                heapq.heappush(todo, Node(curr=(y, x), name=inp[y][x], parent=None, cost=0, dir="S"))

    path = {}
    had = {}

    def print_map():
        for y in range(len(inp)):
            for x in range(len(inp[0])):
                if (y, x) in path:
                    print(path[(y, x)].dir, end="")
                else:
                    print(inp[y][x], end="")
            print()

    while todo:
        curr = heapq.heappop(todo)
        if curr.curr in had:
            continue
        had[curr.curr] = curr
        print(curr.cost, curr.name)

        if curr.name == "E":
            curr.dir = "E"
            c = curr

            while c.parent is not None:
                path[c.curr] = c
                c = c.parent

            print_map()
            print(curr.cost)

            break

        for dir, i in curr.neighbors():
            if i in had:
                continue

            if i[0] < 0 or i[1] < 0 or i[0] >= len(inp) or i[1] >= len(inp[0]):
                continue

            new_name = inp[i[0]][i[1]]

            if (new_name != "E" and ord(new_name) <= ord(curr.name) + 1) or (new_name == "E" and curr.name == "z") or (curr.name == "S" and new_name == "a"):
                new_cost = curr.cost + 1
                node = Node(curr=i, parent=curr, name=new_name, cost=new_cost, dir=dir)

                heapq.heappush(todo, node)



if __name__ == '__main__':
    main()  

# 376