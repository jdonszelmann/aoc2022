print([[i+l for i in range(len(x)-l+1)if len(set(x[i:i+l]))==l][0]for l in[4,14]for x in[open("data.in").read()]])

