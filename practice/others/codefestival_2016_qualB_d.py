n = int(input())
curr_max = 0
ans = 0
for i in range(n):
    a = int(input())
    if i == 0:
        ans += (a-1)
        curr_max = 1
    else:
        step = curr_max + 1
        if a > step: 
            diff = a - 1
            ans += diff//step
        else:
            curr_max = max(a, curr_max)
            # print(i, curr_max)

print(ans)
