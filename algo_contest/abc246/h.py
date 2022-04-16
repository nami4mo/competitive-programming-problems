

def main():
    n = int(input())
    sy, sx = map(int, input().split())
    sy -= 1
    sx -= 1
    gy, gx = map(int, input().split())
    gy -= 1
    gx -= 1

    sl = []
    for _ in range(n):
        row = list(input())
        r = []
        for ri in row:
            if ri == '#':
                r.append(False)
            else:
                r.append(True)
        sl.append(r)

    def ind(y, x):
        return y*n+x

    cnt = 0

    gl = [[] for _ in range(n**2)]  # (to, cost)

    for r in range(-n, n+1):
        curr = []
        ys = 0
        if r < 0:
            ys = r*(-1)
        for d in range(n):
            y = ys+d
            x = y+r
            if x >= n or y >= n:
                break
            if sl[y][x]:
                curr.append((y, x))
            else:
                gl.append([])
                for yy, xx in curr:
                    # print(yy, xx)
                    i = ind(yy, xx)
                    gl[i].append((cnt+n**2, 0))
                    gl[cnt+n**2].append((i, 1))
                cnt += 1
                curr = []
        if curr:
            gl.append([])
            for yy, xx in curr:
                # print(yy, xx)

                i = ind(yy, xx)
                # print(i)

                gl[i].append((cnt+n**2, 0))
                gl[cnt+n**2].append((i, 1))
            cnt += 1

    starts = []
    for i in range(n):
        starts.append((0, i))
    for i in range(1, n):
        starts.append((i, n-1))

    for ssy, ssx in starts:
        curr = []
        for d in range(n):
            y = ssy+d
            x = ssx-d
            if x >= n or y >= n or x < 0 or y < 0:
                break
            if sl[y][x]:
                curr.append((y, x))
            else:
                gl.append([])
                for yy, xx in curr:
                    i = ind(yy, xx)
                    gl[i].append((cnt+n**2, 0))
                    gl[cnt+n**2].append((i, 1))
                cnt += 1
                curr = []
        if curr:
            gl.append([])
            for yy, xx in curr:
                i = ind(yy, xx)
                gl[i].append((cnt+n**2, 0))
                gl[cnt+n**2].append((i, 1))
            cnt += 1

    # print(gl)
    # for row in gl:
    #     print(row)

    import heapq

    INF = 10**18

    def dijkstra(s, n, g):  # s: start, n: |V|, g; glaph (to,cost)
        d = [INF] * n
        # -- record the prev vertex of each one for restoring the route --
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
                    heapq.heappush(que, (d[next_v], next_v))
        return d

    d = dijkstra(ind(sy, sx), len(gl), gl)
    if d[ind(gy, gx)] == INF:
        print(-1)
    else:
        print(d[ind(gy, gx)])
    # print(d)


if __name__ == "__main__":
    main()
