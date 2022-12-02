
def main():
    input_part1 = open("input_part1.txt").read()

    table1 = {
        ('A', 'X'): 4,
        ('A', 'Y'): 8,
        ('A', 'Z'): 3,
        ('B', 'X'): 1,
        ('B', 'Y'): 5,
        ('B', 'Z'): 9,
        ('C', 'X'): 7,
        ('C', 'Y'): 2,
        ('C', 'Z'): 6,
    }

    table2 = {
        ('A', 'X'): 3,
        ('A', 'Y'): 4,
        ('A', 'Z'): 8,
        ('B', 'X'): 1,
        ('B', 'Y'): 5,
        ('B', 'Z'): 9,
        ('C', 'X'): 2,
        ('C', 'Y'): 6,
        ('C', 'Z'): 7,
    }

    total = sum(table1[tuple(i.split(' '))] for i in input_part1.split('\n') if i.strip() != "")
    print(total)


    total = sum(table2[tuple(i.split(' '))] for i in input_part1.split('\n') if i.strip() != "")
    print(total)


if __name__ == '__main__':
    main()  
                