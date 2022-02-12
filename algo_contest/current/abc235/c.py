n, q = map(int, input().split())
al = list(map(int, input().split()))
d = {}
for i, a in enumerate(al):
    d.setdefault(a, [])
    d[a].append(i+1)

for _ in range(q):
    x, k = map(int, input().split())
    if x not in d:
        print(-1)
        continue

    if len(d[x]) < k:
        print(-1)
        continue

    print(d[x][k-1])
