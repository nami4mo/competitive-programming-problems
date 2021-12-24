# 0-indexed
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
            if l&1 == 0:
                v_l = self.seg_f(v_l, self.tree[l])
            if r&1 == 0:
                v_r = self.seg_f(v_r, self.tree[r-1])
                r -= 1
            l >>= 1
            r >>= 1
        return self.seg_f(v_l ,v_r)


'''
memo:
GCD... INFをide_eleに設定して、片方がINFの場合はもう片方を返す
def gcd2(a,b):
    if a >= INF: return b
    elif b >= INF: return a
    else return gcd(a,b)


vl = [1,4,6,8,9]

def add(a,b): return a+b
st = SegTree(vl,add,0)
st = SegTree(vl,(lambda a,b: a+b),0)
st = SegTree(vl,min,10**10)

print(st.query(2,4))
st.update(4,-4)
print(st.query(2,5))
st.update(0,-10)
print(st.query(0,5))
'''

n,L=map(int, input().split())
dp=[10**18]*(L+3)
dp[0]=0
dp=SegTree(dp,min,10**18)
cl=[0]*(L+3)

lights=[]
for _ in range(n):
    l,r,c=map(int, input().split())
    lights.append((l,r,c))

lights.sort(key=lambda x: x[1])
for l,r,c in lights:
    c_min=dp.query(l,r)
    curr=dp.query(r,r+1)
    dp.update(r,min(c_min+c,curr))
    cl[r]=min(c_min+c,curr)

print(dp.query(L,L+1))