from bisect import bisect_left, bisect_right


def p_factorization_osa_k(vl):
    vmax = max(vl)
    min_primes = list(range(vmax+1))  # initialize all values by its own value
    for i in range(2, int(vmax**0.5) + 1):
        if min_primes[i] != i:
            continue  # if not prime
        for j in range(i, vmax+1, i):
            # -> if min_primes[j] == j: min_primes[j] = i
            min_primes[j] = min(min_primes[j], i)
    results = []
    for v in vl:
        p_cnt = {}
        curr_v = v
        while curr_v > 1:
            min_p = min_primes[curr_v]
            p_cnt.setdefault(min_p, 0)
            p_cnt[min_p] += 1
            curr_v //= min_p
        results.append(p_cnt)
    return results


def main():
    n = int(input())
    ps = p_factorization_osa_k(list(range(n+1)))
    # print(ps)

    sqs = []
    for i in range(1, n+1):
        s = i*i
        if s > n:
            break
        sqs.append(s)
    # print(sqs)

    ans = 0
    for i in range(1, n+1):
        pr = 1
        for p, c in ps[i].items():
            if c % 2 == 1:
                pr *= p
        ma = n//pr
        cnt = bisect_right(sqs, ma)
        ans += cnt
    print(ans)


if __name__ == "__main__":
    main()
