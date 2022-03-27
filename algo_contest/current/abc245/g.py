
def main():
    n, m, k, l = map(int, input().split())
    al = list(map(int, input().split()))
    bl = list(map(int, input().split()))
    al = [a-1 for a in al]
    bl = [b-1 for b in bl]

    gl = [[] for _ in range(n)]
    for i in range(n):
        u, v, c = map(int, input().split())
        u -= 1
        v -= 1
        gl[u].append((v, c))
        gl[v].append((u, c))

    from heapq import heappop, heappush

    INF = 10**18
    d = [INF]*n
    d2 = [INF]*n
    que = []  # (a,b): (a: shortest dist, b: node)

    # print(gl)
    # -2: もう完璧、-1: 送りたい、
    vnot = [INF]*n
    for b in bl:
        d[b] = 0
        vnot[b] = al[b]
        heappush(que, (0, b, vnot[b]))

    while que:
        dist, v, c_vnot = heappop(que)
        if c_vnot == -2:
            if d[v] < dist:
                continue
        elif c_vnot == -1:
            for next_v, cost in gl[v]:
                if d2[next_v] < d2[v]+cost and vnot[next_v] == -2:
                    continue
                # if d[next_v] > d[v] + cost:
                d2[next_v] = d2[v] + cost
                heappush(que, (d2[next_v], next_v, c_vnot))
                vnot[next_v] = -1
            vnot[v] = -2
        else:
            for next_v, cost in gl[v]:
                if vnot[next_v] == INF:
                    if d2[next_v] < d[v]+cost and vnot[next_v] == -2:
                        continue
                    if d2[next_v] < d[v]+cost and vnot[next_v] == -1:
                        continue
                    if d2[next_v] < d[v]+cost and vnot[next_v] >= 0 and vnot[next_v] == c_vnot:
                        continue
                    vnot[next_v] = -1
                    d2[next_v] = d[v] + cost
                    heappush(que, (d2[next_v], next_v, c_vnot))

                # d2[next_v] = d[v] + cost
                # heappush(que, (d2[next_v], next_v, c_vnot))
                else:
                    if d[next_v] < d[v]+cost and vnot[next_v] == -2:
                        continue
                    if d[next_v] < d[v]+cost and vnot[next_v] == -1:
                        continue
                    if d[next_v] < d[v]+cost and vnot[next_v] >= 0 and vnot[next_v] == c_vnot:
                        continue
                    vnot[next_v] = c_vnot
                    d[next_v] = d[v] + cost
                    heappush(que, (d[next_v], next_v, c_vnot))

    for i in range(n):
        if d2[i] == INF:
            d2[i] = -1
    print(*d2)


if __name__ == "__main__":
    main()
