
print((lambda s,d:[sum(i for i in d if i<100000),min(i for i in d if i>=-40000000+s)])(*(lambda x,v:x(v,x))(lambda s,f:(sum((r:=list(zip(*[(v,0,[])if type(v)==int else(0,*f(v,f))if n!=".."else(0,0,[])for n,v in s.items()])))[0])+sum(r[1]),sum(r[2],[])+[i for i in r[1]if i!=0]),(lambda s:[__import__("functools").reduce(lambda a,i:(a[i[2]]if i[1]=="cd"else a)if i[0]=="$"else[a.update({i[1]: {"..": a}}),a][1]if "dir"in i else[a.update({i[1]:int(i[0])}),a][1],[i.strip().split(" ")for i in open("data.in").read().split("\n")][1:],s),s][1])({}))))


# print((lambda s, d: [sum(i for i in d if i < 100000), min(i for i in d if i >= -40000000 + s)])(
#     *(lambda x,v:x(v,x))(
#         lambda s,f:(sum((r:=list(zip(*[(v, 0, []) if type(v) == int else (0, *f(v, f)) if n != ".." else (0, 0, []) for n, v in s.items()])))[0])+sum(r[1]),sum(r[2], [])+[i for i in r[1] if i != 0]),
#         (lambda s: [
#             __import__("functools").reduce(
#                 lambda a, i: (a[i[2]] if i[1] == "cd" else a) if i[0] == "$" else [
#                     a.update({i[1]: {"..": a}}),
#                     a
#                 ][1] if "dir" in i else [
#                     a.update({i[1]: int(i[0])}),
#                     a
#                 ][1],
#                 [i.strip().split(" ") for i in open("data.in").read().split("\n")][1:], s
#             ),
#             s
#         ][1])({})
#     )
# ))


