

def main(n, al):
    # al.sort(reverse=True)
    # print(sum(al[:5]))
    bl = al[:]
    bl.sort()
    mid = bl[n//2]

    bl = []
    for a in al:
        if a >= mid:
            bl.append(1)
        else:
            bl.append(0)
    cl = []
    for i in range(n//2):
        c = bl[n-1-i*2]
        c += bl[n-1-i*2-1]
        cl.append(c)

    # print(bl, cl)
    mx = 0
    curr = 0
    for i in range(n//2):
        curr += cl[i]
        diff = (i+1)-curr
        mx = max(mx, diff)
    if mx <= 0:
        return 0
    # print(mx)

    curr = 0
    for i in range(n//2):
        curr += cl[n//2-1-i]
        diff = curr-(i+1)
        if diff >= mx:
            return (i+1)*2
    return 10**10


if __name__ == "__main__":
    n = int(input())
    al = list(map(int, input().split()))

    al1 = al[:]
    al2 = al[:]
    al2 = al2[1:] + al2[0:1]
    al3 = al[:]
    al3.sort(reverse=True)
    ans = sum(al3[:n//2])
    k = main(n, al1)
    if k == 10**10:
        k = main(n, al2)+1
    k %= n
    print(k, ans)
