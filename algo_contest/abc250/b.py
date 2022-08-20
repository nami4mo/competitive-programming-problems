n, a, b = map(int, input().split())
ans1 = []
ans2 = []
for y in range(a):
    row1 = []
    row2 = []
    for x in range(b*n):
        v = x//b
        if v % 2 == 0:
            row1.append('.')
            row2.append('#')
        else:
            row1.append('#')
            row2.append('.')
    ans1.append(''.join(row1))
    ans2.append(''.join(row2))

for i in range(n):
    if i % 2 == 0:
        for r in ans1:
            print(r)
    else:
        for r in ans2:
            print(r)
