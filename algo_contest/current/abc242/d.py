

def main():
    s = int(input())
    q = int(input())
    ql = []
    for _ in range(q):
        t, k = map(int, input().split())
        ql.append((t, k-1))

    for t, k in ql:
        curr = ''
        for i in range(t):


if __name__ == "__main__":
    main()
