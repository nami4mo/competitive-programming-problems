

def main():
    n = int(input())
    xyl = []
    for _ in range(n):
        x, y = map(int, input().split())
        xyl.append((x, y))

    xyl = xyl[:]+xyl[:]

    allsum8 = 0
    for i in range(n):
        x1, y1 = xyl[i]
        x2, y2 = xyl[i+1]
        val = (x1*y2 - x2*y1)*4
        allsum8 += val

    cl = 0
    cr = 2
    csum8 = 0
    for i in range(3):
        x1, y1 = xyl[i]
        r = i+1
        if r == 3:
            r = 0
        x2, y2 = xyl[r]
        val = (x1*y2 - x2*y1)*4
        # print(val)
        csum8 += val

    target = allsum8//4
    ans = 10**60
    # print(allsum8, target)
    for cl in range(n+1):
        # print(cl, cr, csum8)
        while csum8 < target:
            x1, y1 = xyl[cr]
            x2, y2 = xyl[cr+1]
            x3, y3 = xyl[cl]
            val = (x1*y2 - x2*y1)*4
            csum8 += val
            val = (x2*y3 - x3*y2)*4
            csum8 += val
            val = (x1*y3 - x3*y1)*4
            csum8 -= val
            ans = min(ans, abs(csum8-target))
            cr += 1

        x1, y1 = xyl[cr]
        x2, y2 = xyl[cl]
        x3, y3 = xyl[cl+1]
        val = (x1*y2 - x2*y1)*4
        csum8 -= val
        val = (x2*y3 - x3*y2)*4
        csum8 -= val
        val = (x1*y3 - x3*y1)*4
        csum8 += val
        ans = min(ans, abs(csum8-target))

    print(ans)


if __name__ == "__main__":
    main()
