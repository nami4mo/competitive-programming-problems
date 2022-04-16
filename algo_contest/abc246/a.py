x = set()
y = set()
for _ in range(3):
    xi, yi = map(int, input().split())
    if xi in x:
        x.remove(xi)
    else:
        x.add(xi)
    if yi in y:
        y.remove(yi)
    else:
        y.add(yi)

x = list(x)[0]
y = list(y)[0]
print(x, y)
