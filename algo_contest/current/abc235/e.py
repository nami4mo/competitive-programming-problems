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


n, m, q = map(int, input().split())
gl = []  # (cost, a,b,flag,i)
for i in range(m):
    a, b, c = map(int, input().split())
    a -= 1
    b -= 1
    gl.append((c, a, b, 0, i))

for i in range(q):
    a, b, c = map(int, input().split())
    a -= 1
    b -= 1
    gl.append((c, a, b, 1, i))
# qs = []
# for _ in range(q):
#     a, b, c = map(int, input().split())
#     a -= 1
#     b -= 1
#     gl.append((c, a, b))

uf = UnionFind(n)
ans = ['No']*q
gl.sort(key=lambda x: x[0])
for c, a, b, flag, i in gl:
    if flag == 0:
        if uf.same(a, b):
            continue
        uf.union(a, b)
    else:
        if uf.same(a, b):
            ans[i] = 'No'
        else:
            ans[i] = 'Yes'

for a in ans:
    print(a)
