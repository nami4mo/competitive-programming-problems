

def main():
    n, k = map(int, input().split())
    al = list(map(int, input().split()))
    ans = 0
    d = {0: 1}
    csum = 0
    for i in range(n):
        a = al[i]
        csum += a
        v = csum-k
        if v in d:
            ans += d[v]
        d.setdefault(csum, 0)
        d[csum] += 1
    print(ans)


if __name__ == "__main__":
    main()
