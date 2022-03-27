n, m = map(int, input().split())
ans = 0

INF = 10**18
d = [[INF]*n for _ in range(n)]
od = [[INF]*n for _ in range(n)]

for _ in range(m):
    a, b, c = map(int, input().split())
    d[a-1][b-1] = c
    d[b-1][a-1] = c
    od[a-1][b-1] = c
    od[b-1][a-1] = c

for k in range(n):
    for i in range(n):
        for j in range(n):
            d[i][j] = min(d[i][j], d[i][k] + d[k][j])

ans = 0
for i in range(n):
    for j in range(i+1, n):
        if od[i][j] == d[i][j]:
            for k in range(n):
                if k == i or k == j:
                    continue
                if d[i][k]+d[k][j] == d[i][j]:
                    ans += 1
                    break
        elif od[i][j] != INF:
            ans += 1

print(ans)
