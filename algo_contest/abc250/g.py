

def main():
    n = int(input())
    pl = list(map(int, input().split()))

    from heapq import heappop, heappush
    ans = 0
    q = []
    for p in pl:
        if q and q[0] < p:
            poped = heappop(q)
            ans += (p-poped)
            heappush(q, p)
            heappush(q, p)
        else:
            heappush(q, p)
    print(ans)


if __name__ == "__main__":
    main()
