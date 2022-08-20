
from itertools import permutations
from bisect import bisect_left


def lis(n, al):
    INF = 10**18
    dp = [INF]*(n+1)
    dp[0] = -INF

    for i, a in enumerate(al):
        ind = bisect_left(dp, a)  # max index of "value <= a"
        dp[ind] = a
    ans = bisect_left(dp, INF) - 1  # cnt of "value < INF" and remove 0-index
    return ans


def main():
    n, x = map(int, input().split())
    ll = list(range(1, n+1))  # list of elements
    perml = list(permutations(ll, n))

    dic = {}
    for p in perml:
        al = []
        for i in range(n-1):
            a = abs(p[i+1]-p[i])
            al.append(a)
        if p[0] != x:
            continue
        # print(p)
        li = lis(n, al)
        dic.setdefault(li, [])
        dic[li].append((p, al))

    v = list(dic.items())
    v.sort(key=lambda x: -x[0])
    print(v[0][0])
    for vv in v[0][1]:
        print(vv)
        # for m in vv:
        #     print(m)


if __name__ == "__main__":
    main()
