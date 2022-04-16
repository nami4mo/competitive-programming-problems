
from itertools import product


class Combination:
    def __init__(self, n_max=10**6, mod=10**9+7):
        # self._n_max = n_max
        self._fac, self._finv, self._inv = [0]*n_max, [0]*n_max, [0]*n_max
        self._fac[0], self._fac[1] = 1, 1
        self._finv[0], self._finv[1] = 1, 1
        self._inv[1] = 1
        self._mod = mod
        for i in range(2, n_max):
            self._fac[i] = self._fac[i - 1] * i % self._mod
            self._inv[i] = self._mod - self._inv[self._mod % i] * (self._mod // i) % self._mod
            self._finv[i] = self._finv[i - 1] * self._inv[i] % self._mod

    def com(self, n, r):
        if n < r:
            return 0
        if n < 0 or r < 0:
            return 0
        return self._fac[n] * (self._finv[r] * self._finv[n - r] % self._mod) % self._mod

    def perm(self, n, r):
        if n < r:
            return 0
        if n < 0 or r < 0:
            return 0
        return self._fac[n] * (self._finv[n-r] % self._mod) % self._mod

    def lucas(self, n, r):  # nCr (mod self._mod(prime))
        if n < r:
            return 0
        res = 1
        while n > 0:
            nq, rq = n//self._mod, r//self._mod
            nr, rr = n-nq*self._mod, r-rq*self._mod
            res *= self.com(nr, rr)
            res %= self._mod
            n, r = nq, rq
        return res


MOD = 998244353


comb = Combination(40, MOD)


def main():
    n, L = map(int, input().split())
    sl = []
    lis = []
    for _ in range(n):
        ss = list(input())
        if ss in sl:
            continue
        sl.append(ss)

    st = set()
    for s in sl:
        s.sort()
        # print(s)
        ite = list(product(range(2), repeat=len(s)))
        for pattern in ite:
            ss = []
            for i, v in enumerate(pattern):
                if v == 1:
                    ss.append(s[i])
            ss = ''.join(ss)
            st.add(ss)
    st.remove('')

    csum = [0]
    c = 0
    for i in range(1, 28):
        v = pow(i, L, MOD)
        c += v
        c %= MOD
        csum.append(c)
    # print(csum)

    ds = [0]
    for i in range(1, 27):
        val = pow(i, L, MOD)
        for j in range(1, i):
            vv = pow(j, L, MOD)
            vv *= comb.com(i, j)
            vv %= MOD
            if (i-j) % 2 == 1:
                val -= vv
            else:
                val += vv
        ds.append(val % MOD)

    # print(ds)
    # ans = 0
    lis = list(st)
    # for s in lis:
    #     slen = len(s)
    #     val = pow(slen, L, MOD)
    #     val -= csum[slen-1]
    #     val %= MOD
    #     print(s, slen, val)
    #     ans += val
    # print(ans)

    cnts = [0]*27
    for l in lis:
        cnts[len(l)] += 1
    # print(cnts)

    ans = 0
    for i in range(27):
        ans += ds[i]*cnts[i]
    print(ans % MOD)

    # MOD = 998244353
    # ans = 0
    # for i in range(1, 26+1):
    #     val = pow(i, L) * cnts[i]
    #     val %= MOD
    #     val = val - ans
    #     ans += val
    #     ans %= MOD
    #     print(i, ans)

    # print(ans)

    # alps = 'abcdefghijklmnopqrstuvwxyz'  # string.ascii_lowercase
    # ans = []
    # MOD = 998244353
    # for i in range(1, max(l, 26)+1):
    #     if


if __name__ == "__main__":
    main()
