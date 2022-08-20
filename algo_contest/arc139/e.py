

def main():
    n, m = map(int, input().split())
    lis = []
    for x in range(1, n+1):
        for y in range(1, m+1):
            lis.append((x, y))

    from itertools import product
    ite = list(product(range(2), repeat=n*m))

    ok = [[]]
    for pattern in ite:
        v1 = set()
        v2 = set()
        res = []
        for i, v in enumerate(pattern):
            if v == 1:
                x, y = lis[i]
                vv1 = x+3*y
                vv2 = 3*x+y
                if vv1 in v1 or vv2 in v2:
                    break
                v1.add(vv1)
                v2.add(vv2)
                res.append((x, y))
        else:
            if len(ok[0]) < len(res):
                ok.clear()
                res.sort()
                ok.append(res)
            elif len(ok[0]) == len(res):
                ok.append(res)
            else:
                pass
    for v in ok:
        print(v)


if __name__ == "__main__":
    main()
