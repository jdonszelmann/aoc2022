import copy
import heapq
from collections import deque, defaultdict


class GraphNode:
    def __init__(self, name, flow_rate, connections):
        self.name = name
        self.connections = connections
        self.flow_rate = flow_rate

    def __repr__(self):
        return f"{self.name} with rate {self.flow_rate} to {self.connections}"


class Node:
    def __init__(self, graphnode: GraphNode, opened: {str: int}, released: int, time: int, parent, open=None):
        self.graphnode = graphnode
        self.opened = opened
        self.sum_opened = sum(opened.values())
        self.released = released
        self.time = time
        self.parent = parent
        self.open = open

    def potentially_released(self, at):
        diff = at - self.time
        return (self.released + self.sum_opened * diff) * 2

    def __gt__(self, other: "Node"):
        if self.time == other.time:
            return self.released.__lt__(other.released)
        elif self.time > other.time:
            return self.released.__lt__(other.potentially_released(self.time))
        else:
            return self.potentially_released(other.time).__lt__(other.released)

    def __repr__(self):
        return f"minute {self.time}: on {self.graphnode.name} having released {self.released} and opened {self.opened}"


def main():
    inp = [i for i in open("data.test").read().split("\n")]

    nodes = {}

    for i in inp:
        name = i.split(" ")[1]
        rate = int(i.split(" ")[4][5:-1])
        nodes[name] = GraphNode(name, rate, [])

    for i in inp:
        name = i.split(" ")[1]
        res = i.split("valve")[1].removeprefix("s").removeprefix(" ")
        for conn in res.split(","):
            nodes[name].connections.append(conn.strip())

    start = "AA"
    todo = []
    heapq.heappush(todo, Node(nodes[start], {}, 0, 1, None))

    largest = None

    def print_largest_hist():
        if largest is None:
            return

        curr: Node = largest
        path = [curr]
        while curr.parent is not None:
            curr = curr.parent
            path.append(curr)

        assert len(path) == 30

        for idx, i in enumerate(path[::-1]):
            print(f"== Minute {idx + 1} (now in, {i.graphnode.name} having released: {i.released}) ==")
            if len(i.opened) == 0:
                print("No valves are open.")
            else:
                print(f"Valve(s) {[i for i in i.opened.keys()]} are open, releasing {i.sum_opened} pressure")
            if i.open is not None:
                print(f"you open {i.open}")


    while todo:
        curr = heapq.heappop(todo)
        if curr.time >= 30:
            if largest is None or curr.released > largest.released:
                largest = curr
                print_largest_hist()

            continue

        # open
        if curr.graphnode.flow_rate > 0 and curr.graphnode.name not in curr.opened:
            new_opened = copy.deepcopy(curr.opened)
            new_opened[curr.graphnode.name] = curr.graphnode.flow_rate
            new_released = curr.released + curr.sum_opened
            heapq.heappush(todo, Node(curr.graphnode, new_opened, new_released, curr.time + 1, curr, open=curr.graphnode.name))

        # move
        for i in curr.graphnode.connections:
            new_node = nodes[i]
            new_released = curr.released + curr.sum_opened

            heapq.heappush(todo, Node(new_node, curr.opened, new_released, curr.time + 1, curr))


if __name__ == '__main__':
    main()  
                