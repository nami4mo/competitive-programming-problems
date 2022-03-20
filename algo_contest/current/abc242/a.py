a, b, c, x = map(int, input().split())
if x <= a:
    print(1.0)
elif b < x:
    print(0.0)
else:
    ans = c/(b-a)
    print(ans)
