
def main():
    n = int(input())
    pl = list(map(int, input().split()))
    pl = [p-1 for p in pl]

    MOD = 998244353
    # l が root, [l,r] からなる条件を満たす木
    dp0 = [[0]*(n+1) for _ in range(n+1)]
    # l が root, [l,r] からなる条件を満たす木
    # ただし、r が最後に l の直下にちょこんとついたパターン
    dp1 = [[0]*(n+1) for _ in range(n+1)]

    for r in range(n):
        for l in range(n-1, -1, -1):
            if l > r:
                continue
            if r-l <= 1:
                dp0[l][r] = 1
                dp1[l][r] = 1
            else:
                lr0 = dp0[l][r]
                lr1 = dp1[l][r]
                for k in range(l+1, r+1):
                    v0 = dp0[k][r]
                    v1 = dp1[l][k]
                    if k == l:
                        v0 = lr0
                    if k == r:
                        v1 = lr1
                    dp0[l][r] += v1*v0
                    dp0[l][r] %= MOD
                    if pl[k] < pl[r]:
                        dp1[l][r] += v1*v0
                        dp1[l][r] %= MOD
    ans = dp0[0][n-1]
    print(ans)


if __name__ == "__main__":
    main()
