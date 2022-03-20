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


MOD = 10**9+7
comb = Combination(10**6, 2)
comb.com(10, 3)

N = 12
for i in range(1000):
    v = comb.lucas(N+i, i+1)
    if v % 2 == 1:
        print(i)


# for i in range(1, 15):
#     for j in range(1, 10):
#         v = comb.com(i, j)
#         if v % 2 != 0:
#             print(i, j)
