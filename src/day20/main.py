
def mod(a, b):
    res = a % b
    return res if not res else res - b if a < 0 else res

def main():
    key = 811589153
    inp = [[int(i) * key, idx] for idx, i in enumerate(open("data.in").read().split("\n"))]

    for c in range(10):
        print(c)
        for ctr in range(len(inp)):
            for i in range(len(inp)):
                if inp[i][1] == ctr:
                    break

            x = inp.pop(i)
            pos = mod(x[0] + i, len(inp))
            if pos != 0:
                inp.insert(pos, x)
            else:
                inp.append(x)

    inp = [i[0] for i in inp]

    s = inp.index(0)
    a = inp[mod(s + 1000, len(inp))]
    b = inp[mod(s + 2000, len(inp))]
    c = inp[mod(s + 3000, len(inp))]
    print(sum([a, b, c]))
    exit()



if __name__ == '__main__':
    main()


if __name__ == '__main__':
    main()
