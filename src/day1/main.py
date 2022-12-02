
def main():
    inp = [
        sum(int(j) for j in i.split("\n") if j.strip() != "")
        for i in open("data.in").read().split("\n\n")
    ]
    inp.sort()

    part_1 = inp[-1]
    print(part_1)

    part_2 = sum(inp[-3:])
    print(part_2)


if __name__ == '__main__':
    main()

