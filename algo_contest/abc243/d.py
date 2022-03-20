

from dis import dis


def main():
    n, x = map(int, input().split())
    level = 0
    for i in range(100):
        if pow(2, i) > x:
            level = i-1
            break
    dist = x-pow(2, level)
    s = input()
    keep_d = -1
    for si in s:
        if level <= 65:
            prev_d = dist
            if si == 'U':
                level -= 1
                dist //= 2
            elif si == 'L':
                level += 1
                dist = 2*dist
            else:
                level += 1
                dist = 2*dist+1
            if level == 66:
                keep_d = prev_d
        else:
            if si == 'U':
                level -= 1
                # dist //= 2
            elif si == 'L':
                level += 1
                # dist = 2*dist
            else:
                level += 1
                # dist = 2*dist+1
            if level == 65:
                dist = keep_d
        # print(level)
    ans = pow(2, level)+dist
    print(ans)


if __name__ == "__main__":
    main()
