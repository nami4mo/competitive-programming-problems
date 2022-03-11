

MOD = 998244353

p2i = [1]
for i in range(10**6):
    v = p2i[-1]*26
    v %= MOD
    p2i.append(v)


def s2i(s):
    return ord(s) - ord('A')


def solve():
    n = int(input())
    t = input()
    al = []
    for ti in t:
        al.append(s2i(ti))

    if n % 2 == 0:
        ans = 0
        for i in range(n//2):
            num = al[i]
            rem_len = n-(i+1)*2
            ans += p2i[(rem_len+1)//2]*num
            ans %= MOD
        for i in range(n//2):
            left = al[n//2-1-i]
            right = al[n//2+i]
            if left > right:
                break
            elif left < right:
                ans += 1
                break
        else:
            ans += 1
        ans %= MOD
        print(ans)
    else:
        # print(al[n//2])
        ans = 0
        for i in range(n//2):
            num = al[i]
            rem_len = n-(i+1)*2
            ans += p2i[(rem_len+1)//2]*num
            ans %= MOD
        for i in range(1, n//2+1):
            left = al[n//2-i]
            right = al[n//2+i]
            if left > right:
                ans += al[n//2]
                break
            elif left < right:
                ans += al[n//2]
                ans += 1
                break
        else:
            ans += al[n//2]
            ans += 1
        ans %= MOD
        print(ans)


def main():
    t = int(input())
    for _ in range(t):
        solve()


if __name__ == "__main__":
    main()
