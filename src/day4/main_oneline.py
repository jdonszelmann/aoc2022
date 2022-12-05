print([
    sum(m(*x)for x in zip(*[iter([int(x)for d in open("data.in").read().split("\n")for p in d.split(",")for x in p.split("-")])]*4))
    for m in [
        lambda a,b,c,d: c >= a and d <= b or a >= c and b <= d,
        lambda a,b,c,d: a <= d and c <= b
    ]
])

print([sum(m(*x)for x in zip(*[iter([int(x)for d in open("data.in").read().split("\n")for p in d.split(",")for x in p.split("-")])]*4))for m in[lambda a,b,c,d:c>=a and d<=b or a>=c and b<=d,lambda a,b,c,d:a<=d and c<=b]])
