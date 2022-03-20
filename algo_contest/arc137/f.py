n = int(input())
al = list(map(int, input().split()))
dsum = al[0]

for i in range(n-1):
    # d = al[i+1]-al[i]-1
    # dsum += d
    dsum += al[i+1]-i
if dsum == 0:
    print('Bob')
else:
    print('Alice')

# for i in range(n):
#     # al[i] -= i
#     al[i] -= n
#     al[i] = max(al[i], 0)

# v = 0
# for a in al:
#     v ^= a

# if v == 0:
#     print('Bob')
# else:
#     print('Alice')
