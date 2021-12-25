x, y = map(int, input().split())
rem = y-x
ans = (rem-1)//10+1
ans = max(0, ans)
print(ans)
