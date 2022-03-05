n = int(input())
al = list(map(int, input().split()))


def solve_mid(mid):
    bl = []
    for a in al:
        if a < mid:
            bl.append(-1)
        else:
            bl.append(1)
    dp0 = [0]*(n+1)  # prev_use
    dp1 = [0]*(n+1)  # prev_skip
    for i in range(n):
        b = bl[i]
        dp0[i+1] = max(dp0[i]+b, dp1[i]+b)
        dp1[i+1] = dp0[i]
    # thre = 1 if n % 2 == 0 else 0
    if dp0[n] >= 1 or dp1[n] >= 1:
        return True
    else:
        return False


def solve_ave(mid):
    bl = []
    for a in al:
        bl.append(a-mid)
    dp0 = [0]*(n+1)  # prev_use
    dp1 = [0]*(n+1)  # prev_skip
    for i in range(n):
        b = bl[i]
        dp0[i+1] = max(dp0[i]+b, dp1[i]+b)
        dp1[i+1] = dp0[i]
    if dp0[n] >= 0 or dp1[n] >= 0:
        return True
    else:
        return False


ok, ng = 0, 10**9+1
while abs(ok-ng) > 10**(-4):
    mid = (ok+ng)/2
    res = solve_ave(mid)
    # print(mid)
    if res:
        ok = mid
    else:
        ng = mid
print(ok)

ok, ng = 0, 10**9+1
while abs(ok-ng) > 1:
    mid = (ok+ng)//2
    res = solve_mid(mid)
    if res:
        ok = mid
    else:
        ng = mid
print(ok)
