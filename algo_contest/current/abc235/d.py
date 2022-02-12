from collections import deque
a, n = map(int, input().split())
MAX = 10**7
# MAX = 100
dist = [-1]*(10**7)

dist[1] = 0
q = deque()
q.append(1)

while q:
    p = q.popleft()
    v1 = p*a
    v2 = -1
    # print(p)
    if p % 10 != 0 and p > 10:
        bottom = p//10
        s = str(p % 10)+str(bottom)
        v2 = int(s)
    vl = [v1, v2]
    for v in vl:
        if v >= MAX or dist[v] != -1 or v == -1:
            continue
        dist[v] = dist[p]+1
        q.append(v)

print(dist[n])
