
# https://atcoder.jp/contests/abc140/tasks/abc140_e
# https://atcoder.jp/contests/agc005/tasks/agc005_b
# https://atcoder.jp/contests/abc157/tasks/abc157_e

''' 
    [Bit]
'''
class Bit:
    """ used for only int(>=0) 
        0-indexed 
    """
    def __init__(self, n):
        self.size = n
        self.tree = [0] * (n + 1)
        self.depth = n.bit_length()
 
    def _sum(self, i):
        i+=1
        s = 0
        while i > 0:
            s += self.tree[i]
            i -= i & -i
        return s

    def sum(self,l,r):
        return self._sum(r-1)-self._sum(l-1)
 
    def add(self, i, x):
        i+=1
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
        return pos, sum_

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


n=int(input())
hl=[(int(input()),i) for i in range(n)]
hl.sort(reverse=True)
bit=Bit(n)

ansl=[0]*n
prev_h=-1
prev_hil=[]
for h,i in hl:
    if prev_h!=h:
        for hi in prev_hil:
            bit.add(hi,1)
        prev_hil=[]
    
    cnt=bit.sum(0,i)
    if cnt==0:
        ansl[i]+=i
    else:
        wall,_=bit.lower_bound(cnt)
        ansl[i]+=(i-wall-1)
    
    wall,_=bit.lower_bound(cnt+1)
    ansl[i]+=(wall-i-1)

    prev_hil.append(i)
    prev_h=h


for a in ansl:print(a)