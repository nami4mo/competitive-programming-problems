n,m=map(int, input().split())
sl=list(map(int, input().split()))
tl=list(map(int, input().split()))

dp=[[0]*(m+1) for _ in range(n+1)]
dp[0][0]=1
for i in range(n+1):dp[i][0]=1
for j in range(m+1):dp[0][j]=1
MOD=10**9+7

for i in range(1,n+1):
    si=sl[i-1]
    for j in range(1,m+1):
        ti=tl[j-1]
        if si==ti: dp[i][j]+=dp[i-1][j-1]
        dp[i][j]+=dp[i-1][j]
        dp[i][j]+=dp[i][j-1]
        dp[i][j]-=dp[i-1][j-1]
        dp[i][j]%=MOD

print(dp[n][m])