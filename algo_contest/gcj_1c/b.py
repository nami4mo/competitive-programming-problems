
from itertools import permutations
for case in range(int(input())):
    n, p = map(int, input().split())
    al = []
    for _ in range(n):
        row = list(map(int, input().split()))
        row.sort()
        al.append(row)

    INF = 10**18
    dp = [[INF]*(p+1) for _ in range(n+1)]
    dp[0][0] = 0
    # for j in range(p):
    #     dp[1][j] = al[0][j]

    perml = list(permutations(range(p), p))
    for i in range(n):
        for j in range(p):
            if i == 0:
                prev_orig = 0
            else:
                prev_orig = al[i-1][j]
            for perm in perml:
                prev = prev_orig
                dsum = 0
                for k in perm:
                    ak = al[i][k]
                    diff = abs(prev-ak)
                    prev = ak
                    dsum += diff
                dp[i+1][perm[-1]] = min(dp[i+1][perm[-1]], dp[i][j]+dsum)

        # for k in range(p):
        #     ak = al[i][k]
        #     diff = abs(aj-ak)
        #     dp[i+1][k] = min(dp[i+1][k], dp[i][j]+diff)
    # print(dp)
    ans = min(dp[n])

    print('Case  #' + str(case+1) + ': ', ans)
