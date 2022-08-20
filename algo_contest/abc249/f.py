
def main():
    n, k = map(int, input().split())
    tyl = [(1, 0)]
    for _ in range(n):
        t, y = map(int, input().split())
        tyl.append((t, y))

    from heapq import heappop, heappush
    rem = k
    curr = 0
    remove = []
    ans = -10**18
    for t, y in tyl[::-1]:
        if t == 2:
            if y >= 0:
                curr += y
            else:
                if rem > len(remove):
                    heappush(remove, y*(-1))
                elif rem == len(remove) and rem > 0 and remove[0] < y*(-1):
                    poped = heappop(remove)*(-1)
                    curr += poped
                    heappush(remove, y*(-1))
                else:
                    curr += y
        else:
            v = y + curr
            ans = max(ans, v)
            rem -= 1
            if rem < 0:
                break
            if rem < len(remove):
                poped = heappop(remove)*(-1)
                curr += poped

    print(ans)


if __name__ == "__main__":
    main()
