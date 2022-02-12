n = int(input())
al = list(map(int, input().split()))
ans = -1
for a in al:
    if ans < a:
        ans = a
    else:
        break
print(ans)
