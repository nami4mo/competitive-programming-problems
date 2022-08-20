n = int(input())
base = [False]*4
al = list(map(int, input().split()))

ans = 0
for a in al:
    n_base = [False]*4
    # n_base[0] = True
    base[0] = True
    for i in range(4):
        if base[i]:
            if i + a >= 4:
                ans += 1
            else:
                n_base[i+a] = True
    base = n_base[:]
    # print(base)

# print(base)
print(ans)
