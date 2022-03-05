n, m = map(int, input().split())
al = list(map(int, input().split()))
d = {}
for a in al:
    d.setdefault(a, 0)
    d[a] += 1
bl = list(map(int, input().split()))
for b in bl:
    if b not in d or d[b] == 0:
        print('No')
        exit()
    d[b] -= 1
print('Yes')
