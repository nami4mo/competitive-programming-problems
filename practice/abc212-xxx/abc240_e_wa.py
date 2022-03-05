# not-recursive rerooting

# 末端のノードが order で順番に並んでいる必要がありそう
# この order は、そこの保証はされていないので、WA

from collections import deque


def make_order_parents(n, gl, root=0):
    order = [root]
    parents = [-1]*n
    q = deque([root])
    while q:
        poped = q.popleft()
        for neib in gl[poped]:
            if parents[neib] != -1 or neib == root:
                continue
            parents[neib] = poped
            order.append(neib)
            q.append(neib)
    return order, parents


n = int(input())
gl = [[] for _ in range(n)]
for _ in range(n-1):
    a, b = map(int, input().split())
    gl[a-1].append(b-1)
    gl[b-1].append(a-1)
order, pare = make_order_parents(n, gl)

curr = 1
dp = [(10**8, -1)]*n
for v in order[::-1]:
    if pare[v] == -1:
        continue
    # if dp[v] == (10**8, -1):
    if v != 0 and len(gl[v]) == 1:
        dp[v] = (curr, curr)
        curr += 1
    dp[pare[v]] = (min(dp[pare[v]][0], dp[v][0]), max(dp[pare[v]][1], dp[v][1]))

for a, b in dp:
    print(a, b)
