
# 0-indexed
from math import gcd
INF = 3*10**9


class SegTree:
    def __init__(self, vals, seg_f, ide_ele):
        n = len(vals)
        self.seg_f = seg_f
        self.ide_ele = ide_ele
        self.leaf_n = 1 << (n-1).bit_length()
        self.tree = [ide_ele]*(2*self.leaf_n-1)
        for i in range(n):  # init leaf
            self.tree[i+self.leaf_n-1] = vals[i]
        for i in range(self.leaf_n-2, -1, -1):  # bottom-up
            self.tree[i] = self.seg_f(self.tree[2*i+1], self.tree[2*i+2])

    def update(self, i, x):
        i += (self.leaf_n-1)
        self.tree[i] = x
        while i > 0:
            i = (i-1)//2
            self.tree[i] = self.seg_f(self.tree[2*i+1], self.tree[2*i+2])

    # [l,r)
    def query(self, l, r):
        v_l = self.ide_ele
        v_r = self.ide_ele
        l += (self.leaf_n-1)
        r += (self.leaf_n-1)
        while l < r:
            if l & 1 == 0:
                v_l = self.seg_f(v_l, self.tree[l])
            if r & 1 == 0:
                v_r = self.seg_f(v_r, self.tree[r-1])
                r -= 1
            l >>= 1
            r >>= 1
        return self.seg_f(v_l, v_r)


def gcd2(a, b):
    if a >= INF or a == 0:
        return b
    elif b >= INF or b == 0:
        return a
    else:
        return gcd(a, b)

# n = int(input())
# al = list(map(int, input().split()))
# st = SegTree(al, gcd2, INF)
# ans = 0
# for i in range(n):
#     if i != 0:
#         st.update(i-1, al[i-1])
#     st.update(i, INF)
#     v = st.query(0, n)
#     ans = max(ans, v)


# print(ans)


def main():
    n, q = map(int, input().split())
    al = list(map(int, input().split()))
    bl = list(map(int, input().split()))
    ad = []
    bd = []
    for i in range(n-1):
        d = abs(al[i]-al[i+1])
        ad.append(d)
        d = abs(bl[i]-bl[i+1])
        bd.append(d)

    st_a = SegTree(ad, gcd2, INF)
    st_b = SegTree(bd, gcd2, INF)

    for _ in range(q):
        h1, h2, w1, w2 = map(int, input().split())
        h1 -= 1
        h2 -= 1
        w1 -= 1
        w2 -= 1
        val = al[h1] + bl[w1]
        ga = st_a.query(h1, h2)
        gb = st_b.query(w1, w2)

        if ga != INF:
            val = gcd(val, ga)
        if gb != INF:
            val = gcd(val, gb)
        print(val)

    # ans = 0
    # for i in range(n):
    #     if i != 0:
    #         st.update(i-1, al[i-1])
    #     st.update(i, INF)
    #     v = st.query(0, n)
    #     ans = max(ans, v)


if __name__ == "__main__":
    main()
