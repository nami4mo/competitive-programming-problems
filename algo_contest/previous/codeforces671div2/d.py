from collections import deque

n = int(input())
al = list(map(int, input().split()))
al.sort()
q = deque(al)
ans = [-1]*n
for i in range(1,n,2):
    ans[i] = q.popleft()

for i in range(0,n,2):
    ans[i] = q.popleft()

print((n-1)//2)
print(*ans)
