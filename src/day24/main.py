from __future__ import annotations

import functools
from heapq import heappush, heappop
import dataclasses


def add(a, b):
    return a[0] + b[0], a[1] + b[1]


@dataclasses.dataclass
class Blizzard:
    dir: (int, int)
    pos: (int, int)

    def __hash__(self):
        return hash((self.dir, self.pos))


def main():
    inp = [i for i in open("data.in").read().split("\n")]

    blizzards = set()
    start = (inp[0].index("."), 0)
    end = (inp[-1].index("."), len(inp) - 1)

    blizzard_width = len(inp[0]) - 1
    blizzard_height = len(inp) - 1

    def not_in_blizzards(b: set[Blizzard], p: (int, int)) -> bool:
        return not any(Blizzard(pos=p, dir=d) in b for d in [(0, 1), (1, 0), (-1, 0), (0, -1)])

    def in_bounds(p: (int, int)) -> bool:
        return (0 < p[0] < blizzard_width and 0 < p[1] < blizzard_height) or p == start or p == end

    @functools.cache
    def move_blizzards(old_blizzards: frozenset):
        blizzards = set()
        for i in old_blizzards:
            new_pos = list(add(i.pos, i.dir))
            if new_pos[0] >= blizzard_width:
                new_pos[0] = 1
            if new_pos[0] < 1:
                new_pos[0] = blizzard_width-1

            if new_pos[1] >= blizzard_height:
                new_pos[1] = 1
            if new_pos[1] < 1:
                new_pos[1] = blizzard_height-1

            blizzards.add(Blizzard(pos=tuple(new_pos), dir=i.dir))

        return frozenset(blizzards)

    @dataclasses.dataclass
    class Node:
        parent: "Node" | None
        blizzards: frozenset[Blizzard]
        pos: (int, int)
        cost: int
        move: str

        def __lt__(self, other):
            return self.cost < other.cost

        def simulate(self):
            new_blizzards = move_blizzards(self.blizzards)

            for i, n in [((0, 0), "wait"), ((1, 0), "right"), ((-1, 0), "left"), ((0, -1), "up"), ((0, 1), "down")]:
                if not_in_blizzards(new_blizzards, add(i, self.pos)) and in_bounds(add(i, self.pos)):
                    yield Node(
                        parent=self,
                        cost=self.cost + 1,
                        blizzards=new_blizzards,
                        pos=add(i, self.pos),
                        move=n
                    )

        def print(self):
            if self.parent is not None:
                self.parent.print()

            if self.move == "":
                print("Initial state")
            else:
                print(f"minute {self.cost} move: {self.move}")

            for j in range(blizzard_height + 1):
                for i in range(blizzard_width + 1):
                    if (i, j) == self.pos:
                        print("E", end="")
                    elif (i, j) == start:
                        print(".", end="")
                    elif (i, j) == end:
                        print(".", end="")
                    elif i == 0 or j == 0 or i == blizzard_width or j == blizzard_height:
                        print("#", end="")
                    else:
                        c = 0
                        for b in self.blizzards:
                            if b.pos == (i, j):
                                c += 1
                        if c == 0:
                            print(".", end="")
                        elif c == 1:
                            for b in self.blizzards:
                                if b.pos == (i, j):
                                    match b.dir:
                                        case (0, 1):
                                            print("v", end="")
                                        case (0, -1):
                                            print("^", end="")
                                        case (1, 0):
                                            print(">", end="")
                                        case (-1, 0):
                                            print("<", end="")
                        else:
                            print(c, end="")
                print()

            print("=================")

    for y, row in enumerate(inp):
        for x, col in enumerate(row):
            if col == ">":
                blizzards.add(Blizzard(pos=(x, y), dir=(1, 0)))
            if col == "<":
                blizzards.add(Blizzard(pos=(x, y), dir=(-1, 0)))
            if col == "^":
                blizzards.add(Blizzard(pos=(x, y), dir=(0, -1)))
            if col == "v":
                blizzards.add(Blizzard(pos=(x, y), dir=(0, 1)))

    def search(initial, goal):
        todo = []
        heappush(todo, initial)
        had = set()

        while todo:
            curr = heappop(todo)

            if (curr.pos, curr.cost) in had:
                continue
            had.add((curr.pos, curr.cost))

            if curr.pos == goal:
                return curr

            for i in curr.simulate():
                todo.append(i)

    final = search(Node(
        parent=None,
        blizzards=frozenset(blizzards),
        pos=start,
        cost=0,
        move=""
    ), end)
    print("there", final.cost)
    back = search(final, start)
    print("and back again", back.cost)
    again = search(back, end)
    print("and back there again", again.cost)


if __name__ == '__main__':
    main()
