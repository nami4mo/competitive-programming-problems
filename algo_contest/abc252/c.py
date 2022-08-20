n = int(input())
sl = []
for _ in range(n):
    row = list(input())
    aaa = []
    for r in row:
        aaa.append(int(r))
    sl.append(aaa)

ans = 10**10
for num in range(10):
    mods = [0]*10
    for s in sl:
        for i in range(10):
            if s[i] == num:
                mods[i] += 1
                break
    # print(mods)
    cnt = 0
    tmp = 0
    for i in range(10**6):
        t = tmp % 10
        if mods[t] > 0:
            mods[t] -= 1
            cnt += 1
        if cnt == n:
            break
        tmp += 1
    ans = min(ans, tmp)

print(ans)
