
from collections import deque


def main():
    for case in range(int(input())):
        n, k = map(int, input().split())
        vl = list(map(int, input().split()))

        if k == 1:
            a = 0
            b = 0
            for v in vl:
                a += v**2
                b += v
            if b == 0:
                if a == 0:
                    print('Case  #' + str(case+1) + ': 0')
                else:
                    print('Case  #' + str(case+1) + ': IMPOSSIBLE')
                continue
            top = a-b**2
            bottom = 2*b
            if top % bottom == 0:
                x = top//bottom
                print('Case  #' + str(case+1) + ':', x)
            else:
                print('Case  #' + str(case+1) + ': IMPOSSIBLE')
        elif k == 2:
            a = 0
            b = 0
            for v in vl:
                a += v**2
                b += v
            c = b**2+a
            if c % 2 != 0:
                print('Case  #' + str(case+1) + ': IMPOSSIBLE')
                continue
            else:
                c //= 2
                x = c-b
                y = 1-b
                print('Case  #' + str(case+1) + ':', x, y)
        # else:
        #     a = 0
        #     b = 0
        #     for v in vl:
        #         a += v**2
        #         b += v
        #     c = b**2+a
        #     if c % 2 != 0:
        #         print('Case  #' + str(case+1) + ': IMPOSSIBLE')
        #         continue
        #     else:
        #         c //= 2
        #         x = c-b
        #         y = 1-b
        #         print('Case  #' + str(case+1) + ':', x, y)


if __name__ == "__main__":
    main()
