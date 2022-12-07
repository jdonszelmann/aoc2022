def main():
    inp = open("data.in").read().split("\n")

    structure = {}
    path = []
    listing = False

    for i in inp:
        if i.startswith("$"):
            if listing:
                listing = False

            match i.split("$ ")[1].split(" "):
                case ["cd", "/"]:
                    path = []
                case ["cd", ".."]:
                    path.pop()
                case ["cd", *x]:
                    path.append(*x)
                case ["ls"]:
                    listing = True
        elif listing:
            curr = structure
            for x in path:
                curr = curr[x]

            match i.split(" "):
                case ["dir", name]:
                    curr[name] = {}
                case [size, name]:
                    curr[name] = int(size)

    total = 0
    def search(structure: dict[str, dict | int]):
        nonlocal total

        contents = 0

        for name, val in structure.items():
            if isinstance(val, dict):
                res = search(val)
                contents += res

                if res < 100000:
                    total += res
            else:
                contents += val

        return contents

    search(structure)
    print(total)


    def search(structure: dict[str, dict | int], depth = 0):
        contents = 0

        for name, val in structure.items():
            if isinstance(val, dict):
                contents += search(val, depth+2)
            else:
                contents += val

        return contents

    total = search(structure)
    maximum = 70000000
    leftover = abs(total - maximum)
    update_size = 30000000
    needs_leftover = abs(leftover - update_size)
    print(needs_leftover, total)

    dirsizes = []

    def search(structure: dict[str, dict | int]):
        contents = 0

        for name, val in structure.items():
            if isinstance(val, dict):
                dirsize = search(val)
                if dirsize >= needs_leftover:
                    dirsizes.append(dirsize)

                contents += dirsize
            else:
                contents += val

        return contents

    search(structure)
    print(min(dirsizes))


if __name__ == '__main__':
    main()  

