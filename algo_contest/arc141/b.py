

def main():
    n, m = map(int, input().split())
    MOD = 998244353
    MAX = 60

    if n > MAX:
        print(0)
        exit()

    nmax = min(n, MAX)
    dp = [[0]*(MAX+1) for _ in range(n+1)]
    # i 番目の要素のmaxビットが j
    dp[0][0] = 1

    # for i in range(n):
    #     for j in range(MAX):
    #         # pat = min(1 << j, m)
    #         pat = min(pow(2, j+1)-1, m) - pow(2, j) + 1
    #         if j == 0:
    #             pat = min(pat, 1)
    #         print(i, j, pat)
    #         if pat < 0:
    #             continue
    #         for k in range(MAX):
    #             if j > k:
    #                 dp[i+1][j] += dp[i][k]*pat
    #                 dp[i+1][j] %= MOD

    for i in range(n):
        for j in range(1, MAX+1):
            bit = j-1
            pat = min(pow(2, bit+1)-1, m) - pow(2, bit) + 1
            # if j == 0:
            #     pat = min(pat, 1)
            # print(i, j, bit, pat)
            if pat < 0:
                continue
            for k in range(MAX+1):
                if j > k:
                    dp[i+1][j] += dp[i][k]*pat
                    dp[i+1][j] %= MOD

    # for i in range(MAX):
    #     if
    ans = sum(dp[n]) % MOD
    print(ans)
    # print(dp)


if __name__ == "__main__":
    main()
