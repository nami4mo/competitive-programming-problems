n = int(input())
d = {}
xyl = []
for _ in range(n):
    x, y = map(int, input().split())
    xyl.append((x, y))

s = input()

dl = {}
dr = {}
for i in range(n):
    x, y = xyl[i]
    si = s[i]
    if si == 'L':
        dl.setdefault(y, -10**18)
        dl[y] = max(dl[y], x)
    else:
        dr.setdefault(y, 10**18)
        dr[y] = min(dr[y], x)

for k in dl.keys():
    if k not in dr:
        continue
    if dr[k] < dl[k]:
        print('Yes')
        exit()

print('No')
