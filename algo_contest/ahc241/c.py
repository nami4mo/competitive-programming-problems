from collections import deque
n = int(input())
sl = [list(input()) for _ in range(n)]

for y in range(n):
    q = deque()
    cnt = 0
    for x in range(n):
        if sl[y][x] == '#':
            cnt += 1
            q.append(1)
        else:
            q.append(0)
        if len(q) > 6:
            p = q.popleft()
            cnt -= p
        if cnt >= 4:
            print('Yes')
            exit()

for x in range(n):
    q = deque()
    cnt = 0
    for y in range(n):
        if sl[y][x] == '#':
            cnt += 1
            q.append(1)
        else:
            q.append(0)
        if len(q) > 6:
            p = q.popleft()
            cnt -= p
        if cnt >= 4:
            print('Yes')
            exit()

vv = [(0, 0)]
for i in range(1, n):
    vv.append((0, i))
    vv.append((i, 0))

for v in vv:
    q = deque()
    cnt = 0
    i = 0
    while True:
        y = v[0]+i
        x = v[1]+i
        if y >= n or x >= n:
            break
        if sl[y][x] == '#':
            cnt += 1
            q.append(1)
        else:
            q.append(0)
        if len(q) > 6:
            p = q.popleft()
            cnt -= p
        if len(q) >= 6 and cnt >= 4:
            print('Yes')
            exit()
        i += 1


vv = [(0, 0)]
for i in range(1, n):
    vv.append((0, i))
    vv.append((i, n-1))

for v in vv:
    q = deque()
    cnt = 0
    i = 0
    while True:
        y = v[0]+i
        x = v[1]-i
        if y >= n or x < 0:
            break
        if sl[y][x] == '#':
            cnt += 1
            q.append(1)
        else:
            q.append(0)
        if len(q) > 6:
            p = q.popleft()
            cnt -= p
        if len(q) >= 6 and cnt >= 4:
            print('Yes')
            exit()
        i += 1


print('No')
