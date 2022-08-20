import sys
input = sys.stdin.readline


def main():
    h, w = map(int, input().split())
    sl = []
    for _ in range(h):
        row = list(map(int, input().split()))
        sl.append(row)

    ma = -10**18
    ma_y, ma_x = 0, 0
    for y in range(h):
        for x in range(w):
            if sl[y][x] >= ma:
                ma = sl[y][x]
                ma_y = y
                ma_x = x

    dy = max(ma_y+1, h-ma_y)
    dx = max(ma_x+1, w-ma_x)
    print(dy*dx)


if __name__ == "__main__":
    for _ in range(int(input())):
        main()
