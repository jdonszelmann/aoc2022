
def main():
    inp = open("data.in").read()

    for i in range(len(inp) - 14 + 1):
        win = inp[i: i + 14]
        if len(set(win)) == 14:
            print(i + 14)
            break


if __name__ == '__main__':
    main()  
                