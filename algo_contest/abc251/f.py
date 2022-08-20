

def main():
    n, m = map(int, input().split())
    gl = [[] for _ in range(n)]
    for _ in range(m):
        u, v = map(int, input().split())
        u -= 1
        v -= 1
        gl[u].append(v)
        gl[v].append(u)

    pares = [-1]*n
    del_es = []
    from collections import deque
    q = deque()
    q.append((0, -1))  # (node,pare)
    while q:
        c_node, c_pare = q.popleft()
        print(c_node+1, c_pare+1)
        for neib in gl[c_node]:
            if neib == c_pare:
                continue
            print('neib', neib+1)
            if pares[neib] == -1:
                pares[neib] = c_node
                q.append((neib, c_node))
            else:
                del_es.append((pares[neib], neib))
                pares[neib] = c_node
        print([p+1 for p in pares])
    ans = []
    print(pares)
    for i in range(1, n):

        print(i+1, pares[i]+1)


if __name__ == "__main__":
    main()
