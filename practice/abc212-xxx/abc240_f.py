import sys
input = sys.stdin.readline  # remove last '\n'

for _ in range(int(input())):
    n, m = map(int, input().split())
    xyl = []
    for _ in range(n):
        x, y = map(int, input().split())
        xyl.append((x, y))

    val = 0
    ans = xyl[0][0]
    curr_d = 0
    first = True
    for x, y in xyl:
        next_d = curr_d + x*y
        if next_d <= 0 and curr_d > 0 and x < 0 and not first:
            xx = abs(x)
            yy = curr_d//xx
            yy = max(0, yy)
            yy = min(yy, y)
            val2 = val + (curr_d)*yy + x*yy*(yy+1)//2
            ans = max(ans, val2)

        val += (curr_d)*y + x*y*(y+1)//2
        ans = max(ans, val)
        curr_d = next_d
        first = False
    print(ans)
