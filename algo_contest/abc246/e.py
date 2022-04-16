

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
    # print(sl)

    INF = 10**10
    INF = 9
    dist = [[INF]*n for _ in range(n)]
    dist[sy][sx] = 0
    from collections import deque
    q = deque()
    q.append((sy, sx, 0, 0))
    dyxs = [(-1, -1), (-1, 1), (1, -1), (1, 1)]
    while q:
        y, x, dy, dx = q.popleft()
        for ddy, ddx in dyxs:
            ny, nx = y+ddy, x+ddx
            if ny < 0 or ny >= n or nx < 0 or nx >= n:
                continue
            if not sl[ny][nx]:
                continue
            if ddy == dy and ddx == dx:
                if dist[ny][nx] > dist[y][x]:
                    dist[ny][nx] = min(dist[ny][nx],  dist[y][x])
                    q.appendleft((ny, nx, ddy, ddx))
            else:
                if dist[ny][nx] > dist[y][x]+1:
                    dist[ny][nx] = min(dist[ny][nx],  dist[y][x]+1)
                    q.append((ny, nx, ddy, ddx))
        print((y, x), (dy, dx))
        for row in dist:
            print(*row)
        print()
        # print(q)
    for row in dist:
        print(*row)
    # print(dist)
    if dist[gy][gx] == INF:
        print(-1)
    else:
        print(dist[gy][gx])


if __name__ == "__main__":
    main()
