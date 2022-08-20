

def main():
    n, m, k = map(int, input().split())
    MOD = 998244353

    if k == 0:
        ans = pow(m, n, MOD)
        print(ans)
        return

    dp = [[0]*(m+1) for _ in range(n+1)]
    dp[0][0] = 1

    for j in range(1, m+1):
        dp[1][j] = 1

    for i in range(1, n):
        sdp = [0]
        for j in range(m+1):
            v = dp[i][j] + sdp[-1]
            if v > MOD:
                v -= MOD
            sdp.append(v)
        # print(sdp)
        for j in range(1, m+1):
            ma = j-k
            if ma >= 1:
                dp[i+1][j] += sdp[ma+1] - sdp[1]
            mi = j+k
            # print(j, ma, mi)

            if mi <= m:
                dp[i+1][j] += sdp[m+1] - sdp[j+k]
            dp[i+1][j] %= MOD
    ans = 0
    for i in range(1, m+1):
        ans += dp[n][i]
        ans %= MOD
    print(ans)
    # print(dp)


if __name__ == "__main__":
    main()
