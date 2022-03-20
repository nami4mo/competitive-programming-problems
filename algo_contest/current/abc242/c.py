n = int(input())
MOD = 998244353

dp = [[0]*10 for _ in range(n+1)]

for i in range(1, 10):
    dp[1][i] = 1

for i in range(1, n):
    for j in range(1, 10):
        val = dp[i][j-1] + dp[i][j]
        if j != 9:
            val += dp[i][j+1]
        dp[i+1][j] = val % MOD
ans = 0
for i in range(1, 10):
    ans += dp[n][i]

print(ans % MOD)
# print(dp)
