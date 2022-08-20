

def main():
    n, p = map(int, input().split())
    dp = [[0]*(n+1) for _ in range(n+1)]
    sdp = [[0]*(n+2) for _ in range(n+1)]

    dp[0][0] = pow(25, p-2, p)*26
    for i in range(n):
        sdp[0][i+1] = dp[0][0]
    for l in range(2, n):
        for i in range(n):
            dp[l][i+1] += (sdp[l-2][i+1] - sdp[l-2][max(i-8, 0)])*25
            if l >= 3:
                dp[l][i+1] += (sdp[l-3][max(i-8, 0)] - sdp[l-3][max(i-98, 0)])*25
            if l >= 4:
                dp[l][i+1] += (sdp[l-4][max(i-98, 0)] - sdp[l-4][max(i-998, 0)])*25
            if l >= 5:
                dp[l][i+1] += (sdp[l-5][max(i-998, 0)] - sdp[l-5][max(i-9998, 0)])*25
            dp[l][i+1] %= p
        for i in range(0, n+1):
            sdp[l][i+1] = sdp[l][i] + dp[l][i]
            sdp[l][i+1] %= p
    ans = 0
    for l in range(1, n):
        ans += dp[l][n]
        ans %= p
    print(ans)


if __name__ == "__main__":
    main()
