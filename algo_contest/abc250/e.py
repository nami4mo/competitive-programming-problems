

def main():
    n = int(input())
    al = list(map(int, input().split()))
    bl = list(map(int, input().split()))
    INF = 10**10
    # pa = [INF]*n
    # pb = [INF]*n
    pa = {}
    pb = {}
    for i in range(n):
        if al[i] in pa:
            continue
        pa[al[i]] = i
    for i in range(n):
        if bl[i] in pb:
            continue
        pb[bl[i]] = i

    # print(pa, pb)

    cma = [-1]*(n)
    cmb = [-1]*(n)
    ma_a = -1
    ma_b = -1
    for i in range(n):
        a = al[i]
        pb.setdefault(a, INF)
        ma_a = max(ma_a, pb[a])
        cma[i] = ma_a
        b = bl[i]
        pa.setdefault(b, INF)
        ma_b = max(ma_b, pa[b])
        cmb[i] = ma_b

    # print(cma, cmb)
    for _ in range(int(input())):
        x, y = map(int, input().split())
        x -= 1
        y -= 1
        if cma[x] <= y and cmb[y] <= x:
            print('Yes')
        else:
            print('No')


if __name__ == "__main__":
    main()
