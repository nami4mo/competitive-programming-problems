

from operator import le


def main():
    n = int(input())
    s = input()

    cnts = [0]*n
    for i in range(1, n-1):
        if s[i-1:i+2] == 'ARC':
            cnt = 0
            left, right = i-1, i+1
            while left >= 0 and right <= n-1:
                if s[left] == 'A' and s[right] == 'C':
                    cnt += 1
                else:
                    break
                left -= 1
                right += 1
            cnts[cnt] += 1
    # print(cnts)

    ans = 0
    left = 1
    right = n-1
    while True:
        # print()
        while cnts[right] == 0 and right > 0:
            right -= 1
        if right == 0:
            break

        cnts[right] -= 1
        if right - 1 != 0:
            cnts[right-1] += 1
        ans += 1

        # print(cnts)

        if cnts[left-1] > 0:
            left -= 1
        while cnts[left] == 0 and left < n-1:
            left += 1
        if left == n-1:
            break

        cnts[left] -= 1
        ans += 1

        # print(cnts)

    print(ans)


if __name__ == "__main__":
    main()
