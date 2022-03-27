

def main():
    MAX = 10**5+1
    dp = [0]*MAX
    dp[1] = 1
    for i in range(2, MAX):
        sq = int(i**0.5)
        dp[i] = sum(dp[0:sq+1])

    # for i in range(110):
    #     print(i, dp[i])

    csum = []
    c = 0
    for i in range(MAX):
        c += dp[i]
        csum.append(c)

    # print(dp[:10])
    # print(csum[:10])

    for _ in range(int(input())):
        x = int(input())
        if x < MAX:
            # print(x, dp[x])
            print(dp[x])
            continue

        if x < 10**10:
            sq = int(x**0.5)
            # ans = sum(dp[:sq+1])
            ans = csum[sq]
            # print(x, ans)
            print(ans)
            continue

        if x == 10**10:
            ans = csum[10**5]
            print(ans)

        ans = sum(dp)  # ~ 10**5
        sq = int((10**5)**0.5)
        # xsqmax = int(x**0.5)

        ok = 0
        ng = 10**12
        while ng-ok > 1:
            mid = (ok+ng)//2
            if mid**2 <= x:
                ok = mid
            else:
                ng = mid
        xsqmax = ok

        # print('---', sq, xsqmax)
        for i in range(sq-1, 10**5+1):
            left = max(MAX, i**2)
            right = min(xsqmax, (i+1)**2-1)
            cnt = right - left+1
            # print(i, cnt)
            # print(i, left, right, right**2)
            if cnt <= 0:
                continue
            # ans += sum(dp[:i+1])*cnt
            ans += csum[i]*cnt
        print(ans)


if __name__ == "__main__":
    main()
