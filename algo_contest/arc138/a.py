
from bisect import bisect_left, bisect_right


def main():
    n, k = map(int, input().split())
    al = list(map(int, input().split()))
    bl = al[:k]
    cl = []
    cmin = 10**10
    for i in range(k-1, -1, -1):
        a = al[i]
        cmin = min(a, cmin)
        a = min(a, cmin)
        cl.append(a)
    cl = cl[::-1]
    # print(cl)

    ans = 10**10
    for i in range(k, n):
        a = al[i]
        ind = bisect_left(cl, a) - 1
        ind = ind if 0 <= ind < n else None
        if ind is None:
            continue
        cnt = k-1-ind + i-k+1
        ans = min(cnt, ans)
    if ans == 10**10:
        ans = -1
    print(ans)


if __name__ == "__main__":
    main()
