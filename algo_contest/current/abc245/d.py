

def main():
    n, m = map(int, input().split())
    al = list(map(int, input().split()))
    cl = list(map(int, input().split()))
    bl = []

    min0a = 0
    min0i = 0
    for i in range(n+1):
        if al[i] != 0:
            min0a = al[i]
            min0i = i
            break

    for i in range(n+m+1):
        v = cl[i]
        for j in range(i):
            if j+1 <= n and 0 <= i-(j+1) < len(bl):
                a = al[j+1]
                b = bl[i-(j+1)]
                v -= a*b
        if i < min0i:
            continue

        bi = v//min0a
        bl.append(bi)

    print(*bl[:m+1])


if __name__ == "__main__":
    main()
