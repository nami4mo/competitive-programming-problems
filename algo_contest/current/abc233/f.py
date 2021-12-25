
from collections import deque
import heapq

INF = 10**18


def dijkstra(s, n, g, goal):  # s: start, n: |V|, g; glaph (to,cost)
    INF = 10**18
    d = [INF] * n
    # -- record the prev vertex of each one for restoring the route --
    prev_vl = [-1]*n
    d[s] = 0
    que = []  # (a,b): (a: shortest dist, b: node)
    heapq.heappush(que, (0, s))

    while que:
        dist, v = heapq.heappop(que)
        if d[v] < dist:
            continue  # if v has been already used -> continue
        for next_v, cost in g[v]:
            if d[next_v] > d[v] + cost:
                d[next_v] = d[v] + cost
                prev_vl[next_v] = v
                heapq.heappush(que, (d[next_v], next_v))
        # resotre the route
    # goal = n-1  # set goal here (do you need loop?)
    route = [goal]
    curr_v = goal
    while True:
        prev_v = prev_vl[curr_v]
        if prev_v == -1:
            break
        route.append(prev_v)
        curr_v = prev_v
    route = route[::-1]
    return d, route
    # return d


def get_route(s, n, g, goal):  # s: start, n: |V|, g; glaph (to,cost)
    q = deque()
    q.append(s)
    prev = [-1]*n
    prev[s] = -2
    while q:
        poped = q.popleft()
        # print(poped, '--')
        for neib in g[poped]:
            if prev[neib] != -1:
                continue
            prev[neib] = poped
            q.append(neib)
        if prev[goal] != -1:
            break

    # print(prev)
    if prev[goal] == -1:
        print(-1)
        exit()

    route = [goal]
    curr_v = goal
    while True:
        prev_v = prev[curr_v]
        if prev_v == -2:
            break
        route.append(prev_v)
        curr_v = prev_v
    route = route[::-1]
    return route


def main():
    n = int(input())
    pl = list(map(int, input().split()))
    pl = [p-1 for p in pl]

    edge = [[-1]*n for i in range(n)]
    pos = [-1]*n
    for i in range(n):
        pos[pl[i]] = i

    m = int(input())
    gl = [[] for _ in range(n)]
    gl2 = [[] for _ in range(n)]

    for i in range(m):
        a, b = map(int, input().split())
        a -= 1
        b -= 1
        gl[a].append(b)
        gl[b].append(a)
        gl2[b].append((a, 1))
        gl2[a].append((b, 1))
        edge[a][b] = i
        edge[b][a] = i

    ans = []
    for i in range(n):
        if pos[i] == i:
            continue
        route = get_route(pos[i], n, gl, i)
        for i in range(len(route)-1):
            v1, v2 = route[i], route[i+1]
            pl[v1], pl[v2] = pl[v2], pl[v1]
            pos[pl[v1]], pos[pl[v2]] = pos[pl[v2]], pos[pl[v1]]
            ans.append(edge[v1][v2]+1)
        route = route[::-1]
        for i in range(len(route)-2):
            v1, v2 = route[i+1], route[i+2]
            pl[v1], pl[v2] = pl[v2], pl[v1]
            pos[pl[v1]], pos[pl[v2]] = pos[pl[v2]], pos[pl[v1]]
            ans.append(edge[v1][v2]+1)
    print(len(ans))
    print(*ans)


if __name__ == "__main__":
    main()
