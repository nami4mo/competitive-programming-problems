v, a, b, c = map(int, input().split())
s = a+b+c
v %= s
if v < a:
    print('F')
    exit()
v -= a
if v < b:
    print('M')
    exit()
v -= b
print('T')
