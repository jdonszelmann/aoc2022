
print((lambda o,p,y,r:[len({i[-1]for i in r(lambda s,d:[*s,r(lambda u,v:[*u,min((p(i,v)for i in o[1:]),key=lambda i:y(i,u[-1]))]if all(p(v,a)!=u[-1]for a in o)else[*u,v],s[-1][1:],[p(s[-1][0],d)])],(o[1:]["RLDU".index(d)]for(d, n)in[i.split()for i in open("data.in").read().split("\n")]for _ in range(int(n))),[[(0, 0)for _ in range(l)]])})for l in[2,10]])([(0,0),(0,1),(0,-1),(-1,0),(1,0),(-1,1),(1,1),(1,-1),(-1,-1)],lambda a,b:(a[0]+b[0],a[1]+b[1]),lambda a,b:(abs(a[0]-b[0]),abs(a[1]-b[1])),__import__("functools").reduce))

print(
    (lambda o, p, y, r:[len({i[-1] for i in r(lambda s, d: [
    *s,
    r(lambda u, v:
           [*u, min((p(i, v) for i in o[1:]), key=lambda i: y(i, u[-1]))]
           if all(p(v, a) != u[-1] for a in o)
           else [*u, v],
           s[-1][1:],
      [p(s[-1][0], d)])
], (o[1:]["RLDU".index(d)] for (d, n) in [i.split() for i in open("data.in").read().split("\n")] for _ in range(int(n))), [[(0, 0) for _ in range(l)]])}) for l in [2, 10]])
([(0,0),(0,1),(0,-1),(-1,0),(1,0),(-1,1),(1,1),(1,-1),(-1,-1)],lambda a,b:(a[0]+b[0],a[1]+b[1]),lambda a,b:(abs(a[0]-b[0]),abs(a[1]-b[1])),__import__("functools").reduce))

