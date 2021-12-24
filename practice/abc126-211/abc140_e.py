
''' 
    [Bit]
'''
class Bit:
    """ used for only int(>=0) 
        1-indexed (ignore 0-index)
    """
    def __init__(self, n):
        self.size = n
        self.tree = [0] * (n + 1)
        self.depth = n.bit_length()
 
    def sum(self, i):
        s = 0
        while i > 0:
            s += self.tree[i]
            i -= i & -i
        return s
 
    def add(self, i, x):
        while i <= self.size:
            self.tree[i] += x
            i += i & -i

    def lower_bound(self, x):
        """ 累積和がx以上になる最小のindexと、その直前までの累積和 """
        sum_ = 0
        pos = 0
        for i in range(self.depth, -1, -1):
            k = pos + (1 << i)
            if k <= self.size and sum_ + self.tree[k] < x:
                sum_ += self.tree[k]
                pos += 1 << i
        return pos + 1, sum_

    def get_less_than_x_cnt(self, x):
        """ 累積和がx未満 の個数 """
        lb_pos, lb_sum = self.lower_bound(x)
        return lb_pos-1

    def get_less_than_and_x_cnt(self, x):
        """ 累積和がx以下 の個数 """
        lb_pos, lb_sum = self.lower_bound(x+1)
        return lb_pos-1
    
    def get_more_than_x_cnt(self, x):
        """ 累積和がxより大きい 個数 """
        return self.size - self.get_less_than_and_x_cnt(x)


n = int(input())
pl = list(map(int, input().split()))
pll = []
for i,p in enumerate(pl):
    pll.append((p,i+1))

pll.sort(reverse=True)
bit = Bit(n)
ans = 0
for p,i in pll:
    ix = bit.sum(i)
    # print(ix)
    if ix == 0:
        lmax = 0
        lmin = 0
        lcnt = 0
    else:
        lmax, _ = bit.lower_bound(ix)
        if ix == 1:
            lmin = 0
        else:
            lmin, _ = bit.lower_bound(ix-1)
        lcnt = lmax - lmin
    
    rmin, _ = bit.lower_bound(ix+1)
    rmax, _ = bit.lower_bound(ix+2)
    rcnt = rmax - rmin

    comb = lcnt*(rmin-i) + (i-lmax)*rcnt
    ans += comb*p

    bit.add(i,1)
    # print((p,i))
    # print(lmin,lmax,i,rmin,rmax,'->',comb)
    # print()

print(ans)