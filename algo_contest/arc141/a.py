

def main():
    n = int(input())
    nlen = len(str(n))
    ns = str(n)

    ans = 10**(len-1)-1
    for l in range(1, 19):
        if l >= nlen:
            continue
        if nlen % l != 0:
            continue

        v1 = int(ns[:l]*(nlen//l))
        # print(' ', l, v1)

        if v1 <= n:
            ans = max(ans, v1)
        else:
            val = int(ns[:l])
            val -= 1
            v2 = int(str(val)*(nlen//l))
            ans = max(ans, v2)

    print(ans)


if __name__ == "__main__":
    for _ in range(int(input())):
        main()
