n, x = map(int, input().split())
d = {1: 1}
for _ in range(n):
    al = list(map(int, input().split()))
    al = al[1:]
    new_d = {}
    for num, cnt in d.items():
        for a in al:
            v = num*a
            new_d.setdefault(v, 0)
            new_d[v] += cnt
    d.clear()
    for k, v in new_d.items():
        d[k] = v

    # print(d)
if x in d:
    print(d[x])
else:
    print(0)
