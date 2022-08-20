n = int(input())
dic = {}
for i in range(n):
    s, t = map(str, input().split())
    t = int(t)
    if s in dic:
        continue
    dic[s] = (t, i)

vals = list(dic.items())
# print(vals)
vals.sort(key=lambda x: (-x[1][0], x[1][1]))
print(vals[0][1][1]+1)
