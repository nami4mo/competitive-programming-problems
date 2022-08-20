n, q = map(int, input().split())
al = list(range(1, n+1))
pl = [-1]*(n+1)
for i in range(n):
    pl[i+1] = i

for _ in range(q):
    x = int(input())
    pos = pl[x]
    if pos != n-1:
        x2 = al[pos+1]
        al[pos], al[pos+1] = al[pos+1], al[pos]
        pl[x] = pos+1
        pl[x2] = pos
    else:
        x2 = al[pos-1]
        al[pos], al[pos-1] = al[pos-1], al[pos]
        pl[x] = pos-1
        pl[x2] = pos
print(*al)
