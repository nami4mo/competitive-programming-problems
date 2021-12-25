from functools import lru_cache
import heapq

INF = 10**18

g = [[] for _ in range(10**3)]


@lru_cache(None)
def dijkstra(s, n, goal):  # s: start, n: |V|, g; glaph (to,cost)
    INF = 10**18
    d = [INF] * n
    # -- record the prev vertex of each one for restoring the route --
    prev_vl = [-1]*n
    d[s] = 0
    que = []  # (a,b): (a: shortest dist, b: node)
    heapq.heappush(que, (0, s))

    while que:
        dist, v = heapq.heappop(que)
        if v == goal:
            break
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
        gl[a].append(b)
        gl2[b].append((a, 1))
        gl2[a].append((b, 1))
        g[b].append((a, 1))
        g[a].append((b, 1))
        edge[a][b] = i
        edge[b][a] = i

    ans = []
    for i in range(n):
        if pos[i] == i:
            continue
        d, route = dijkstra(pos[i], n, i)
        if d[i] == INF:
            print(-1)
            exit()
        swapped_val = pl[i]
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
        # print('--', i)
        # print(route, d)
        # print(pl)
    print(len(ans))
    print(*ans)


if __name__ == "__main__":
    main()
