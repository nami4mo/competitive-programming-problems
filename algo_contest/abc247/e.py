

def main():
    n, x, y = map(int, input().split())
    al = list(map(int, input().split()))
    mal = [0]*n
    mil = [0]*n
    ngl = [0]*n

    cma, cmi, cng = -1, -1, n
    for i in range(n-1, -1, -1):
        a = al[i]
        if a < y or a > x:
            cng = i
        elif a == x:
            cma = i
        if a == y:
            cmi = i
        mal[i] = cma
        mil[i] = cmi
        ngl[i] = cng

    ans = 0
    for i in range(n):
        if ngl[i] == i:
            continue
        if mil[i] == -1 or mal[i] == -1:
            continue

        near, far = mil[i], mal[i]
        if near > far:
            near, far = far, near
        if ngl[near] < far:
            continue
        if ngl[i] < near:
            continue
        ok = ngl[far]-far
        ans += ok

    print(ans)


if __name__ == "__main__":
    main()
