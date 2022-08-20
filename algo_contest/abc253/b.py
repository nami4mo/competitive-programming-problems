h, w = map(int, input().split())
sl = []
for _ in range(h):
    row = input()
    sl.append(row)

yy, xx = -1, -1

for y in range(h):
    for x in range(w):
        if sl[y][x] == 'o':
            if yy == -1:
                yy, xx = y, x
            else:
                ans = abs(yy-y)+abs(xx-x)

print(ans)
