n = int(input())
t = input()
x, y = 0, 0

dx, dy = 1, 0
for ti in t:
    if ti == 'S':
        x += dx
        y += dy
    else:
        if (dx, dy) == (1, 0):
            dx, dy = 0, -1
        elif (dx, dy) == (0, -1):
            dx, dy = -1, 0
        elif (dx, dy) == (-1, 0):
            dx, dy = 0, 1
        elif (dx, dy) == (0, 1):
            dx, dy = 1, 0


print(x, y)
