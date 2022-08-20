

def main():
    n = int(input())
    al = list(map(int, input().split()))

    INF = 10**18
    # 0あげないとき
    dp0 = [INF]*n  # あげた
    dp1 = [INF]*n  # あげてない
    dp1[0] = 0
    for i in range(1, n):
        a = al[i]
        dp0[i] = min(dp0[i], dp0[i-1]+a, dp1[i-1]+a)
        dp1[i] = min(dp1[i], dp0[i-1])
    ans = dp0[n-1]

    dp0 = [INF]*n  # あげた
    dp1 = [INF]*n  # あげてない
    dp0[0] = al[0]
    for i in range(1, n):
        a = al[i]
        dp0[i] = min(dp0[i], dp0[i-1]+a, dp1[i-1]+a)
        dp1[i] = min(dp1[i], dp0[i-1])
    ans = min(ans, dp0[n-1], dp1[n-1])
    print(ans)


if __name__ == "__main__":
    main()
