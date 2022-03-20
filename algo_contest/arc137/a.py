from math import gcd
l, r = map(int, input().split())
if gcd(l, r) == 1:
    print(r-l)
    exit()

ans = 0
for i in range(10**3):
    ll = l+i
    for j in range(10**3):
        rr = r-j
        if gcd(ll, rr) == 1:
            ans = max(ans, rr-ll)
print(ans)
