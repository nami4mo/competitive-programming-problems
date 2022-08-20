from bisect import bisect_left, bisect_right


def primes(n):
    is_prime = [True] * (n + 1)
    is_prime[0] = False
    is_prime[1] = False
    for i in range(2, int(n**0.5) + 1):
        if not is_prime[i]:
            continue
        for j in range(i * 2, n + 1, i):
            is_prime[j] = False
    return [i for i in range(n + 1) if is_prime[i]]


def main():
    n = int(input())
    MAX = 10**6+10
    # MAX = 100
    ps = primes(MAX)
    # print(ps)
    ans = 0
    for q in ps:
        div = n//(q**3)
        # print(div)
        ma = min(div, q-1)
        # print(div, ma)
        cnt = bisect_right(ps, ma)
        # print(div, ma, cnt)
        ans += cnt
    print(ans)


if __name__ == "__main__":
    main()
