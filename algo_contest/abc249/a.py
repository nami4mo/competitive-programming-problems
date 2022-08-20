a, b, c, d, e, f, x = map(int, input().split())

ta = 0
ao = 0
for i in range(x):
    if i % (a+c) < a:
        ta += b
    if i % (d+f) < d:
        ao += e

if ta > ao:
    print('Takahashi')
elif ta < ao:
    print('Aoki')
else:
    print('Draw')
# print(ta, ao)
