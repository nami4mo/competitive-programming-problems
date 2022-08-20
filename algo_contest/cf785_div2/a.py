
import sys
input = sys.stdin.readline


def main():
    MAX = 4*10**4
    # MAX = 10
    al = []
    for i in range(1, 10**5):
        si = str(i)
        ri = si[::-1]
        if si == ri:
            al.append(i)
    n = len(al)
    dp = [0]*(MAX+1)
    # print(n)
    # print(al[:100])
    MOD = 10**9+7
    dp[0] = 1
    for i in range(n):
        next_dp = [0]*(MAX+1)
        a = al[i]
        for j in range(MAX+1):
            if j < a:
                next_dp[j] += dp[j]
                next_dp[j] %= MOD
            else:
                next_dp[j] += dp[j]
                next_dp[j] += next_dp[j-a]
                next_dp[j] %= MOD
        dp = next_dp[:]
    # print(dp[:10])
    for _ in range(int(input())):
        v = int(input())
        ans = dp[v]
        print(ans)


if __name__ == "__main__":
    main()
