import colorama


def main():
    inp = [i for i in open("data.in").read().split("\n")]

    mapping = {}
    visible = set()

    height = len(inp)
    width = len(inp[0])

    for i in range(height):
        for j in range(width):
            h = int(inp[i][j])
            mapping[(i, j)] = h

    def print_grid():
        for i in range(5):
            for j in range(5):
                v = (i, j) in visible
                if v:
                    print(f"{colorama.Fore.RED}{mapping[(i, j)]}{colorama.Style.RESET_ALL}", end="")
                else:
                    print(f"{mapping[(i, j)]}", end="")

            print()

        print("------")

    def search_left(row):
        largest = mapping[(row, 0)]
        visible.add((row, 0))
        for i in range(1, width):
            if mapping[(row, i)] > largest:
                visible.add((row, i))
            largest = max(largest, mapping[(row, i)])

    def search_right(row):
        largest = mapping[(row, width-1)]
        visible.add((row, width-1))
        for i in range(width-1, 0, -1):
            if mapping[(row, i)] > largest:
                visible.add((row, i))
            largest = max(largest, mapping[(row, i)])

    def search_top(col):
        largest = mapping[(0, col)]
        visible.add((0, col))
        for i in range(1, height):
            if mapping[(i, col)] > largest:
                visible.add((i, col))
            largest = max(largest, mapping[(i, col)])

    def search_bottom(col):
        largest = mapping[(height - 1, col)]
        visible.add((height-1, col))
        for i in range(height-1, 0, -1):
            if mapping[(i, col)] > largest:
                visible.add((i, col))
            largest = max(largest, mapping[(i, col)])

    # print_grid()
    for i in range(height):
        search_left(i)
        search_right(i)
    for i in range(width):
        search_top(i)
        search_bottom(i)

    # print_grid()
    print(len(visible))

    scores = {}
    for i in range(width):
        for j in range(height):
            h = mapping[(i, j)]
            sl = set()
            sr = set()
            st = set()
            sb = set()
            for x in range(i+1, width):
                sl.add((x, j))
                lm = mapping[(x, j)]
                if lm >= h:
                    break
            for x in range(i-1, -1, -1):
                sr.add((x, j))
                if mapping[(x, j)] >= h:
                    break
            for y in range(j-1, -1, -1):
                sb.add((i, y))
                if mapping[(i, y)] >= h:
                    break
            for y in range(j+1, height):
                st.add((i, y))
                if mapping[(i, y)] >= h:
                    break

            score = len(sr) * len(sl) * len(sb) * len(st)
            scores[(i, j)] = score

    print(scores)
    print(max(scores.values()))

if __name__ == '__main__':
    main()  
                