
def main():
    inp = open("data.in").read()

    t = """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"""

    prio = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

    total = 0

    for i in inp.split("\n"):
        c1 = i[:len(i) // 2]
        c2 = i[len(i) // 2:]

        for v in c1:
            if v in c2:
                total += prio.index(v) + 1
                break
    print(total)

    total = 0
    group = []
    for i in inp.split("\n"):
        group.append(i)
        if len(group) == 3:

            for v in group[0]:
                if v in group[1] and v in group[2]:
                    total += prio.index(v) + 1
                    break

            group = []

    print(total)


if __name__ == '__main__':
    main()  
