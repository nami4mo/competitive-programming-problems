from itertools import product
from bisect import bisect_left, bisect_right


n,t = map(int, input().split())
al = list(map(int, input().split()))
if n%2 == 1:
    n+=1
    al.append(0)

len1 = n//2
len2 = n-len1

al1 = al[:n//2]
al2 = al[n//2:]

al1s = []
al2s = []
ite = list(product(range(2),repeat=n//2))
for pattern in ite:
    curr_sum1 = 0
    curr_sum2 = 0

    for i, v in enumerate(pattern):
        if v == 1:
            curr_sum1 += al1[i]
            curr_sum2 += al2[i]

    al1s.append(curr_sum1)
    al2s.append(curr_sum2)


# al2s = []
# ite = list(product(range(2),repeat=len2))
# for pattern in ite:
#     curr_sum = 0
#     for i, v in enumerate(pattern):
#         if v == 1:
#             curr_sum += al2[i]
#     al2s.append(curr_sum)

# print(al1s)

al1s.sort()
# al2s.sort(reverse=True)
al2s.sort()


# print(al1s)
# print(al2s)
l2 = len(al2s) 

ans = 0
l = 0
for v in al1s:
    rem = t-v
    if rem < 0: continue
    elif rem == 0:
        ans = t
        break
    ind = bisect_right(al2s, rem) - 1
    if ind == -1: continue
    v2 = al2s[ind]
    cans = v+v2
    ans = max(ans,cans)
print(ans)