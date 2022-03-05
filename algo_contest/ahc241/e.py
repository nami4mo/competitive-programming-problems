from math import log2
# n, x, m = map(int, input().split())

n, k = map(int, input().split())
al = list(map(int, input().split()))

logk = int(log2(k))+1
db = [[0]*n for _ in range(logk)]
dbs = [[0]*n for _ in range(logk)]

# a=i なら 次は (i**2)%m
for i in range(n):
    db[0][i] = (i+al[i]) % n
    dbs[0][i] = al[i]

for i in range(logk-1):
    for j in range(n):
        db[i+1][j] = db[i][db[i][j]]
        dbs[i+1][j] = dbs[i][db[i][j]] + dbs[i][j]

ans = 0
now = 0
for i in range(logk):
    if (k) & (1 << i) > 0:
        ans += dbs[i][now]
        now = db[i][now]
# print(dbs)
print(ans)
