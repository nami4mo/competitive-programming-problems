sl = list(map(str, input().split()))
tl = list(map(str, input().split()))

d = 0
for i in range(3):
    if sl[i] != tl[i]:
        d += 1

if d == 0 or d == 3:
    print('Yes')
else:
    print('No')
