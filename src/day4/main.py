
def main():
    inp = [i.split(",") for i in open("data.in").read().split("\n") if i.strip() != ""]

    def parse_pair(p):
        return [int(i) for i in p.split("-")]

    total = 0
    for (a, b) in inp:
        s1, e1 = parse_pair(a)
        s2, e2 = parse_pair(b)
        print(s1, e1, s2, e2)

        if s2 >= s1 and e2 <= e1:
            total += 1
        elif s1 >= s2 and e1 <= e2:
            total += 1
    print(total)

    total = 0
    for (a, b) in inp:
        s1, e1 = parse_pair(a)
        s2, e2 = parse_pair(b)


        if s2 <= s1 <= e2:
            total += 1
        elif s2 <= e1 <= e2:
            total += 1
        elif s1 <= s2 <= e1:
            total += 1
        elif s1 <= e2 <= e1:
            total += 1

    print(total)


if __name__ == '__main__':
    main()  
                