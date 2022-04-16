

def main():
    Q = int(input())
    from collections import deque
    q = deque()
    for i in range(Q):
        ql = list(map(int, input().split()))
        if ql[0] == 1:
            q.append((ql[1], ql[2]))
        else:
            rem = ql[1]
            res = 0
            while q:
                if q[0][1] <= rem:
                    x, c = q.popleft()
                    res += x*c
                    rem -= c
                else:
                    x, c = q.popleft()
                    res += x*rem
                    q.appendleft((x, c-rem))
                    break
            print(res)


if __name__ == "__main__":
    main()
