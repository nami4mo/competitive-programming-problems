s = input()

alps = 'abcdefghijklmnopqrstuvwxyz'  # string.ascii_lowercase
ALPS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'  # string.ascii_uppercase

ok1 = False
ok2 = False
for si in s:
    if si in alps:
        ok1 = True
    else:
        ok2 = True

if not (ok1 and ok2):
    print('No')
    exit()

for si in s:
    cnt = s.count(si)
    if cnt > 1:
        print('No')
        exit()

print('Yes')
