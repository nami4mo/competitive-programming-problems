a, b, c = map(int, input().split())

l = [a, b, c]
l.sort()
if l[1] == b:
    print('Yes')
else:
    print('No')
