

def main():
    n = int(input())
    al = list(map(int, input().split()))
    from collections import deque
    q = deque(al)

    flip = 0
    while q:
        if q[-1] == 0 ^ flip:
            q.pop()
        elif q[0] == 0 ^ flip:
            q.popleft()
            flip ^= 1
        else:
            print('No')
            exit()
    print('Yes')


if __name__ == "__main__":
    main()
