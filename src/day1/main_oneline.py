
print((lambda i: (i[-1], sum(i[-3:])))(sorted([sum(int(j) for j in i.split("\n") if j.strip() != "")for i in open("input_part1.txt").read().split("\n\n")])))
