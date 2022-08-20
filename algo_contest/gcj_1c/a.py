
from collections import deque


def main():
    for case in range(int(input())):
        n = int(input())
        al = list(map(int, input().split()))
        prev = 0
        ma = 0
        q = deque(al)
        ans = 0
        for _ in range(n):
            if q[0] <= q[-1]:
                poped = q.popleft()
                if ma <= poped:
                    ans += 1
                ma = max(ma, poped)
            else:
                poped = q.pop()
                if ma <= poped:
                    ans += 1
                ma = max(ma, poped)
            # if prev <= q[0] <= q[-1]:
            #     poped = q.popleft()
            #     ans += 1
            #     prev = poped
            # elif prev <= q[-1] <= q[0]:
            #     poped = q.pop()
            #     ans += 1
            #     prev = poped
            # elif q[0] <= q[-1] <= prev:
            #     poped = q.popleft()
            #     prev = poped
            # elif q[0] <= prev <= q[-1]:
            #     poped = q.pop()
            #     ans += 1
            #     prev = poped
            # elif q[1] <= q[0] <= prev:
            #     poped = q.pop()
            #     prev = poped
            # elif q[-1] <= prev <= q[0]:
            #     poped = q.popleft()
            #     ans += 1
            #     prev = poped
        print('Case  #' + str(case+1) + ': ', ans)


if __name__ == "__main__":
    main()
