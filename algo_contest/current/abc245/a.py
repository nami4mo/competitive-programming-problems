a, b, c, d = map(int, input().split())

t = a*100+b
ao = c*100+d

if t <= ao:
    print('Takahashi')
else:
    print('Aoki')
