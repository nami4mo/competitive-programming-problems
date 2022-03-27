n, k = map(int, input().split())
al = list(map(int, input().split()))
bl = list(map(int, input().split()))

dp = [al[0], bl[0]]
for i in range(1, n):
    a = al[i]
    b = bl[i]
    next_dp = []

    for v in dp:
        if abs(a-v) <= k:
            next_dp.append(a)
            break
    for v in dp:
        if abs(b-v) <= k:
            next_dp.append(b)
            break

    dp = next_dp[:]

if dp:
    print('Yes')
else:
    print('No')
