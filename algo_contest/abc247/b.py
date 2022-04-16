

n = int(input())
stl = []
for i in range(n):
    s, t = map(str, input().split())
    stl.append((s, t))

for i in range(n):
    si, ti = stl[i]
    sok, tok = True, True
    for j in range(n):
        if i == j:
            continue
        if si == stl[j][0] or si == stl[j][1]:
            sok = False
        if ti == stl[j][0] or ti == stl[j][1]:
            tok = False
    if (not sok) and (not tok):
        print('No')
        exit()
print('Yes')
