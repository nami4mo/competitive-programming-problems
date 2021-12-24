import sys
input=sys.stdin.readline
from collections import defaultdict

class segtree:
    def __init__(self,n,op,e):
        self.e=e
        self.op=op
        self.size=1<<(n-1).bit_length()
        self.SEG=[self.e]*(self.size*2)
    
    def update(self,i,x):
        i+=self.size
        self.SEG[i]=x
        
        while i>0:
            i>>=1
            self.SEG[i]=self.op(self.SEG[i*2],self.SEG[i*2+1])
    
    def get(self,i):
        return self.SEG[i+self.size]
    
    def query(self,l,r):
        l+=self.size
        r+=self.size
        
        lres,rres=self.e,self.e
        
        while l<r:
            if l&1:
                lres=self.op(lres,self.SEG[l])
                l+=1
            
            if r&1:
                r-=1
                rres=self.op(self.SEG[r],rres)
            
            l>>=1
            r>>=1
            
        res=self.op(lres,rres)
        return res

e=(1,0)
def op(a,b):
    return (a[0]*b[0],b[0]*a[1]+b[1])

N,M=map(int,input().split())
query=[tuple(map(float,input().split())) for _ in range(M)]
arr=sorted(set([query[i][0] for i in range(M)]))
d={}
N=len(arr)
for i in range(N):
    d[arr[i]]=i

seg=segtree(N,op,e)
Min,Max=1,1
for p,a,b in query:
    p=d[int(p)]
    seg.update(p,(a,b))
    r=sum(seg.query(0,N))
    print(seg.query(0,N))
    Min=min(Min,r)
    Max=max(Max,r)

print(Min)
print(Max)