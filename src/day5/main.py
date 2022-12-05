import itertools


def parse_table(tbl):
    return [[j[1] for j in reversed(i) if j != ""] for i in itertools.zip_longest(
        *[i.replace("    ", " ").split(" ") for i in list(tbl.split("\n")) if any(j.isalpha() for j in i)],
        fillvalue="")]


def main():
    start, data = open("data.in").read().split("\n\n")
    inp = [i.split(" ") for i in data.split("\n")]

    stacks = parse_table(start)
    print(stacks)

    for _, num, _, src, _, dst in inp:
        tmp = []
        for _ in range(int(num)):
            tmp.append(stacks[int(src) - 1].pop())

        # for part 1, don't reverse
        tmp.reverse()
        stacks[int(dst) - 1].extend(tmp)

    print("".join(i[-1] for i in stacks))


if __name__ == '__main__':
    main()
