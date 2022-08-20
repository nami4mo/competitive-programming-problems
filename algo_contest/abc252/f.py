from heapq import heappop, heappush


def main():
    n, l = map(int, input().split())
    al = list(map(int, input().split()))

    rem = l-sum(al)
    ans = 0
    if rem != 0:
        q = [rem]
    else:
        q = []
    for a in al:
        heappush(q, a)

    while len(q) >= 2:
        v1 = heappop(q)
        v2 = heappop(q)
        ans += v1
        ans += v2
        heappush(q, v1+v2)
    print(ans)

    # for a in al:
    #     print(a)


if __name__ == "__main__":
    main()
