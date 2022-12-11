
class Monkey:
    def __init__(self, starting, op, test, iftrue, iffalse):
        self.items = starting
        self.op = op
        self.test = test
        self.iftrue = iftrue
        self.iffalse = iffalse
        self.inspects = 0


def main():
    inp = [i for i in open("data.in").read().split("\n\n")]

    monkeys = []

    for i in inp:
        starting = []
        op = lambda old: old
        test = 0
        iftrue = 0

        for x in i.split("\n"):
            x = x.strip()
            if x.startswith("Starting items: "):
                starting = [int(i) for i in x[16:].split(", ")]
            elif x.startswith("Operation:"):
                op = eval(f"lambda old: {x[17:]}")
            elif x.startswith("Test: "):
                test = int(x[19:])
            elif x.startswith("If true"):
                iftrue = int(x[25:].strip())
            elif x.startswith("If false"):
                iffalse = int(x[26:].strip())
                monkeys.append(Monkey(starting, op, test, iftrue, iffalse))

    res = 1
    for i in monkeys:
        res *= i.test

    for r in range(10_000):
        for curr in range(len(monkeys)):
            monkey = monkeys[curr]
            olditems = monkey.items
            monkey.items = []
            for item in olditems:
                # print(f"Monkey {curr} inspects item with level {item}")
                new_item = monkey.op(item) % res
                if new_item % monkey.test == 0:
                    monkeys[monkey.iftrue].items.append(new_item)
                else:
                    monkeys[monkey.iffalse].items.append(new_item)
                monkey.inspects += 1

        print(f"Round {r}")
        for idx, i in enumerate(monkeys):
            print(f"Monkey {idx}", i.items)
        print("----")

    monkeys.sort(key=lambda i: i.inspects, reverse=True)

    print(monkeys[0].inspects * monkeys[1].inspects)


if __name__ == '__main__':
    main()  
                