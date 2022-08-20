from math import gcd


def main():
    n, a, b = map(int, input().split())
    acnt = n//a
    bcnt = n//b
    ab = a*b//gcd(a, b)
    abcnt = n//ab

    asum = a*acnt*(acnt+1)//2
    bsum = b*bcnt*(bcnt+1)//2
    absum = ab*abcnt*(abcnt+1)//2

    notsum = asum+bsum-absum
    allsum = n*(n+1)//2
    ans = allsum-notsum
    print(ans)


if __name__ == "__main__":
    main()
