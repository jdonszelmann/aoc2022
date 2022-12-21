from __future__ import annotations

from collections import deque

time = 32


def eval_blueprint(ore_robot_ore, clay_robot_ore, obsidian_robot_ore, obsidian_robot_clay, geode_robot_ore,
                   geode_robot_obsidian, checks=True):
    todo = deque()

    ore_value = ore_robot_ore
    clay_value = clay_robot_ore
    obsidian_value = ore_value * obsidian_robot_ore + clay_value * obsidian_robot_clay
    geode_value = ore_value * geode_robot_ore + obsidian_value * geode_robot_obsidian

    class Node:
        def __init__(self,
                     ore_collecting: int, clay_collecting: int, obsidian_collecting: int, geode_collecting: int,
                     ore: int, clay: int, obsidian: int, geode: int,
                     depth: int
                     ):
            self.ore_collecting = ore_collecting
            self.clay_collecting = clay_collecting
            self.obsidian_collecting = obsidian_collecting
            self.geode_collecting = geode_collecting

            self.ore = ore
            self.clay = clay
            self.obsidian = obsidian
            self.geode = geode

            self.depth = depth

    todo.append(Node(
        1, 0, 0, 0,
        0, 0, 0, 0,
        1
    ))

    max_geode = 0
    best_geode = 0
    last_depth = 0
    i = 0

    max_ore_need = max([ore_robot_ore, geode_robot_ore, obsidian_robot_ore, clay_robot_ore])

    while todo:
        i += 1
        curr: Node = todo.popleft()

        if curr.clay_collecting > obsidian_robot_clay:
            continue
        if curr.ore_collecting > max_ore_need:
            continue
        if curr.obsidian_collecting > geode_robot_obsidian:
            continue
        if curr.geode + ((time + 1) - curr.depth) < best_geode:
            continue

        best_geode = max(best_geode, curr.geode)

        next_ore = curr.ore + curr.ore_collecting
        next_clay = curr.clay + curr.clay_collecting
        next_obsidian = curr.obsidian + curr.obsidian_collecting
        next_geode = curr.geode + curr.geode_collecting

        if i % 1000 == 0:
            print(f"\r{curr.depth}  {len(todo)} {best_geode}", end="")

        if curr.depth != last_depth and last_depth > 2:
            if len(todo) > 0:
                max_geode = max(i.geode_collecting for i in todo)
                todo = deque(i for i in todo if i.geode_collecting >= max_geode)

        last_depth = curr.depth

        if curr.depth >= time:
            max_geode = max(max_geode, next_geode)
            continue

        if curr.ore >= geode_robot_ore and curr.obsidian >= geode_robot_obsidian:
            todo.append(Node(
                curr.ore_collecting, curr.clay_collecting, curr.obsidian_collecting, curr.geode_collecting + 1,
                                                                                     next_ore - geode_robot_ore,
                next_clay, next_obsidian - geode_robot_obsidian, next_geode,
                                                                                     curr.depth + 1
            ))
            continue

        if curr.ore >= obsidian_robot_ore and curr.clay >= obsidian_robot_clay:
            todo.append(Node(
                curr.ore_collecting, curr.clay_collecting, curr.obsidian_collecting + 1, curr.geode_collecting,
                                                           next_ore - obsidian_robot_ore,
                                                           next_clay - obsidian_robot_clay, next_obsidian, next_geode,
                                                           curr.depth + 1
            ))
            continue

        if curr.ore >= ore_robot_ore:
            todo.append(Node(
                curr.ore_collecting + 1, curr.clay_collecting, curr.obsidian_collecting, curr.geode_collecting,
                next_ore - ore_robot_ore, next_clay, next_obsidian, next_geode,
                curr.depth + 1
            ))

        if curr.ore >= clay_robot_ore:
            todo.append(Node(
                curr.ore_collecting, curr.clay_collecting + 1, curr.obsidian_collecting, curr.geode_collecting,
                 next_ore - clay_robot_ore, next_clay, next_obsidian, next_geode,
                 curr.depth + 1
            ))

        todo.append(Node(
            curr.ore_collecting, curr.clay_collecting, curr.obsidian_collecting, curr.geode_collecting,
            next_ore, next_clay, next_obsidian, next_geode,
            curr.depth + 1
        ))

    print()
    print("DONE")
    print(max_geode)
    return max_geode


def main():
    inp = open("data.in").read().split("\n")[:3]
    print(len(inp))
    total = 1
    for idx, i in enumerate(inp):
        parts = [int(j.split(" ")[1]) for i in i.split("costs")[1:] for j in i.split("and")]
        ore_robot_ore, clay_robot_ore, obsidian_robot_ore, obsidian_robot_clay, geode_robot_ore, geode_robot_obsidian = parts
        res = eval_blueprint(ore_robot_ore, clay_robot_ore, obsidian_robot_ore, obsidian_robot_clay, geode_robot_ore,
                             geode_robot_obsidian)
        total *= res

    print("FINAL")
    print(total)
    # 15010

if __name__ == '__main__':
    main()
