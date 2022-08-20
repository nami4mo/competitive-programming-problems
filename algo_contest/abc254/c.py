n, k = map(int, input().split())
al = list(map(int, input().split()))

bl = [[] for _ in range(k)]

for i in range(n):
    bl[i % k].append(al[i])

for j in range(k):
    bl[j].sort()

prev = -1
for i in range(n):
    a = bl[i % k][i//k]
    if a < prev:
        print('No')
        exit()
    prev = a
print('Yes')
