s = list(input())
n = len(s)
ans = n
for i in range(n-1):
    if s[i] != s[i+1]:
        v = max(i+1,n-i-1)
        ans = min(ans,v)
print(ans)