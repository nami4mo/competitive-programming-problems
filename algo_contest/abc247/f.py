

class UnionFind():
    def __init__(self, n):
        self.n = n
        self.parents = [-1] * n
        # if parents[i]>=0 => parents[i] is parent of i
        # if parents[i]<0 => i is root and (-1)*parents[i] shows the group size.

    def find(self, x):
        if self.parents[x] < 0:
            return x
        else:
            self.parents[x] = self.find(self.parents[x])
            return self.parents[x]

    def union(self, x, y):
        x = self.find(x)
        y = self.find(y)

        if x == y:
            return

        # if size[x] < size[y]: => swap (x group: larger, y group: smaller)
        # to merge the smaller group(y) into the larger(x)
        if self.parents[x] > self.parents[y]:
            x, y = y, x
        self.parents[x] += self.parents[y]
        self.parents[y] = x

    def size(self, x):
        return -self.parents[self.find(x)]

    def same(self, x, y):
        return self.find(x) == self.find(y)

    def members(self, x):
        root = self.find(x)
        return [i for i in range(self.n) if self.find(i) == root]

    def roots(self):
        return [i for i, x in enumerate(self.parents) if x < 0]

    def group_count(self):
        return len(self.roots())

    # too slow
    def all_group_members(self):
        return {r: self.members(r) for r in self.roots()}

    def get_all_members(self):
        pare_d = {}
        for i in range(self.n):
            parent = self.find(i)
            pare_d.setdefault(parent, [])
            pare_d[parent].append(i)
        return list(pare_d.values())

    def __str__(self):
        return '\n'.join('{}: {}'.format(r, self.members(r)) for r in self.roots())


def main():
    n = int(input())
    pl = list(map(int, input().split()))
    ql = list(map(int, input().split()))
    uf = UnionFind(n)
    for p, q in zip(pl, ql):
        p -= 1
        q -= 1
        if p == q:
            continue
        uf.union(p, q)

    gs = uf.get_all_members()

    MOD = 998244353
    cans = 1
    for g in gs:
        ans = 0
        n = len(g)
        if n == 1:
            continue
        # 最初の要素を選ぶとき
        dp0 = [0]*(n+1)  # 前選んでない
        dp1 = [0]*(n+1)  # 前選んでる
        dp1[1] = 1
        for i in range(1, n):
            dp0[i+1] = dp1[i]
            dp1[i+1] = dp0[i]+dp1[i]
            dp0[i+1] %= MOD
            dp1[i+1] %= MOD
        ans += dp0[n]
        ans += dp1[n]

        # 最初の要素を選ばない時（最後は絶対選ぶ）
        dp0 = [0]*(n+1)  # 前選んでない
        dp1 = [0]*(n+1)  # 前選んでる
        dp0[1] = 1
        for i in range(1, n):
            dp0[i+1] = dp1[i]
            dp1[i+1] = dp0[i]+dp1[i]
            dp0[i+1] %= MOD
            dp1[i+1] %= MOD
        # ans += dp0[n]
        ans += dp1[n]
        ans %= MOD
        cans *= ans
        cans %= MOD
    print(cans)


if __name__ == "__main__":
    main()
