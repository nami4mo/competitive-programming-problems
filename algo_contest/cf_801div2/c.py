import sys
input = sys.stdin.readline


def main():
    t = int(input())
    for _ in range(t):
        h, w = map(int, input().split())
        al = []
        for _ in range(h):
            row = list(map(int, input().split()))
            al.append(row)

        if (h+w-1) % 2 != 0:
            print('NO')
            continue

        bl = [[0]*w for _ in range(h)]
        for y in range(h):
            for x in range(w):
                if al[y][x] == 1:
                    bl[y][x] = 1

        INF = 0
        dp = [[INF]*w for _ in range(h)]
        dp[0][0] = bl[0][0]
        for y in range(h):
            for x in range(w):
                v = dp[y][x]
                if y != h-1:
                    added = bl[y+1][x]
                    dp[y+1][x] = max(dp[y+1][x], v+added)
                if x != w-1:
                    added = bl[y][x+1]
                    dp[y][x+1] = max(dp[y][x+1], v+added)

        v1 = dp[h-1][w-1]

        bl = [[0]*w for _ in range(h)]
        for y in range(h):
            for x in range(w):
                if al[y][x] == -1:
                    bl[y][x] = 1

        INF = 0
        dp = [[INF]*w for _ in range(h)]
        dp[0][0] = bl[0][0]
        for y in range(h):
            for x in range(w):
                v = dp[y][x]
                if y != h-1:
                    added = bl[y+1][x]
                    dp[y+1][x] = max(dp[y+1][x], v+added)
                if x != w-1:
                    added = bl[y][x+1]
                    dp[y][x+1] = max(dp[y][x+1], v+added)

        v2 = dp[h-1][w-1]

        target = (h+w-1)//2
        if v1 >= target and v2 >= target:
            print('YES')
        else:
            print('NO')


if __name__ == "__main__":
    main()
