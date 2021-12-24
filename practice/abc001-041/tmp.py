dp=[-1]*(5*(10**5)+1)
dp[0]=0
dp[1]=1
MOD=10**9+7
def rec(i,dp):
    if i==0:return 0
    if i==1:return 1
    if dp[i]!=-1:return dp[i]
    dp[i]=rec(i-1,dp)+rec(i-2,dp)
    dp[i]%=MOD
    return dp[i]

rec(5*(10**5),dp)
