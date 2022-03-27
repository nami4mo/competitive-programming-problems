

def main():
    n, m = map(int, input().split())
    gl = [[] for _ in range(n)]
    r_gl = [[] for _ in range(n)]
    dirs = [0]*n
    for _ in range(m):
        u, v = map(int, input().split())
        u -= 1
        v -= 1
        gl[u].append(v)
        r_gl[v].append(u)
        dirs[u] += 1

    from collections import deque
    q = deque()
    for i in range(n):
        if dirs[i] == 0:
            q.append(i)

    while q:
        poped = q.popleft()
        for neib in r_gl[poped]:
            dirs[neib] -= 1
            if dirs[neib] == 0:
                q.append(neib)

    ans = 0
    for i in range(n):
        if dirs[i] > 0:
            ans += 1
    print(ans)


if __name__ == "__main__":
    main()
