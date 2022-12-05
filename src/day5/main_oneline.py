
print(["".join(i[-1]for i in(lambda s,d:__import__("functools").reduce(lambda a,v:[a[int(v[5])-1].extend(m(a[int(v[3])-1][-int(v[1]):])),[a[int(v[3])-1].pop()for _ in range(int(v[1]))],a][-1],[i.split(" ")for i in d.split("\n")],[[j[1]for j in reversed(i)if j!=""]for i in __import__("itertools").zip_longest(*[i.replace("    ", " ").split(" ")for i in list(s.split("\n"))if any(j.isalpha()for j in i)],fillvalue="")]))(*open("data.in").read().split("\n\n")))for m in[reversed,lambda i:i]])


print([
    "".join(i[-1] for i in (
        lambda s, d: __import__("functools").reduce(
            lambda a, v: [a[int(v[5]) - 1].extend(m(a[int(v[3]) - 1][-int(v[1]):])),
                          [a[int(v[3]) - 1].pop() for _ in range(int(v[1]))], a][-1],
            [i.split(" ") for i in d.split("\n")],
            [
                [j[1] for j in reversed(i) if j != ""]
                for i in __import__("itertools").zip_longest(
                *[i.replace("    ", " ").split(" ") for i in list(s.split("\n"))
                  if any(j.isalpha() for j in i)], fillvalue="")
            ])
    )(*open("data.in").read().split("\n\n"))
    ) for m in [reversed, lambda i: i]
])
