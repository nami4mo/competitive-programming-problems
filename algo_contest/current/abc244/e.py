

def main():
    n, m, k, s, t, x = map(int, input().split())
    s -= 1
    t -= 1
    x -= 1
    gl = [[] for _ in range(n)]
    for _ in range(m):
        u, v = map(int, input().split())
        u -= 1
        v -= 1
        gl[u].append(v)
        gl[v].append(u)
    dp0 = [[0]*n for _ in range(k+1)]
    dp1 = [[0]*n for _ in range(k+1)]
    dp0[0][s] = 1
    MOD = 998244353
    for i in range(k):
        for j in range(n):
            for neib in gl[j]:
                if neib != x:
                    dp0[i+1][neib] += dp0[i][j]
                    dp1[i+1][neib] += dp1[i][j]
                    dp0[i+1][neib] %= MOD
                    dp1[i+1][neib] %= MOD
                else:
                    dp0[i+1][neib] += dp1[i][j]
                    dp1[i+1][neib] += dp0[i][j]
                    dp0[i+1][neib] %= MOD
                    dp1[i+1][neib] %= MOD

    print(dp0[k][t])


if __name__ == "__main__":
    main()
