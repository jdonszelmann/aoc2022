

def compare_pair(l, r):
    if isinstance(l, int) and isinstance(r, int):
        print(f"comparing {l} and {r}")
        if l < r:
            return True
        elif l == r:
            return None
        else:
            return False

    elif isinstance(l, list) and isinstance(r, list):
        print(f"comparing list {l} and {r}")

        try:
            for x, y in zip(l, r, strict=True):
                res = compare_pair(x, y)
                if res:
                    return True
                elif res is None:
                    continue
                else:
                    return False
        except ValueError:
            if len(r) > len(l):
                return True
            elif len(l) > len(r):
                return False

        return None

    elif isinstance(l, list):
        print(f"upcasting r {r} to [{r}]")
        return compare_pair(l, [r])
    elif isinstance(r, list):
        print(f"upcasting l {l} to [{l}]")
        return compare_pair([l], r)
    else:
        raise NotImplemented


def main():
    inp = [i for i in open("data.in").read().split("\n\n")]

    oks = []
    total = 0
    for idx, i in enumerate(inp):
        left, right = i.split("\n")
        lp, rp = eval(left), eval(right)

        if compare_pair(lp, rp):
            print(f"> {idx + 1} ordered")
            total += (idx + 1)
            oks.append(idx)
        else:
            print(f"> {idx + 1} not ordered")
    print(oks)
    print(total)

    packets = [eval(j) for i in inp for j in i.split("\n")]
    packets.append([[2]])
    packets.append([[6]])

    for i in range(len(packets)):
        for j in range(len(packets)):
            if compare_pair(packets[i], packets[j]):
                packets[i], packets[j] = packets[j], packets[i]

    a = packets.index([[2]]) + 1
    b = packets.index([[6]]) + 1
    print(a * b)


if __name__ == '__main__':
    main()  
                