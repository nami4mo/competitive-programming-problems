al = list(map(int, input().split()))
hl = al[0:3]
wl = al[3:6]

ans = 0
for v00 in range(1, 31):
    for v01 in range(1, 31):
        for v10 in range(1, 31):
            for v11 in range(1, 31):
                v02 = hl[0]-v00-v01
                v12 = hl[1]-v10-v11
                v20 = wl[0]-v00-v10
                v21 = wl[1]-v01-v11
                v22_0 = hl[2] - v20-v21
                v22_1 = wl[2] - v02-v12
                if v22_0 != v22_1:
                    continue
                if v02 <= 0 or v12 <= 0 or v20 <= 0 or v21 <= 0 or v22_0 <= 0:
                    continue
                ans += 1
                # print(v00, v01, v02)
                # print(v10, v11, v12)
                # print(v20, v21, v22_0)
                # print()

print(ans)
