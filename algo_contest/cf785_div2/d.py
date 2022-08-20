
from math import gcd

import sys
input = sys.stdin.readline


def lcm(x, y): return (x * y) // gcd(x, y)


def solve():
    b, bd, bn = map(int, input().split())
    c, cd, cn = map(int, input().split())
    b -= c
    c -= c  # c の初項を 0 に合わせる

    # print(b, bd, bn)
    # print(c, cd, cn)

    c_last = c+cd*(cn-1)
    b_last = b+bd*(bn-1)

    if cd % bd != 0:
        print(0)
        return

    if (c-b) % bd != 0:
        print(0)
        return
    i = (c-b)//bd
    if i < 0 or i >= bn:
        print(0)
        return

    if (c_last-b) % bd != 0:
        print(0)
        return
    i = (c-b)//bd
    if i < 0 or i >= bn:
        print(0)
        return

    c_last = c+cd*(cn-1)
    b_last = b+bd*(bn-1)
    if c < b or b_last < c_last:
        print(0)
        return

    if c-cd < b or b_last < c_last+cd:
        print(-1)
        return

    divs = []
    for i in range(1, int(cd**0.5)+1):
        if cd % i == 0:
            divs.append(i)
            if i*i != cd:
                divs.append(cd//i)
    ans = 0
    MOD = 10**9+7
    for ad in divs:
        if lcm(ad, bd) != cd:
            continue
        pat = (cd//ad)**2
        ans += pat
        ans %= MOD
    print(ans)
    return


def main():
    for _ in range(int(input())):
        solve()


if __name__ == "__main__":
    main()
