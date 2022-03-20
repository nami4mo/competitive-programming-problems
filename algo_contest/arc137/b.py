n = int(input())
al = list(map(int, input().split()))
cnt = al.count(1)

curr_min = 0
curr_max = 0
curr = 0
for a in al:
    if a == 0:
        a -= 1
    curr += a
    curr_min = min(curr_min, curr)
    v = curr - curr_min
    curr_max = max(curr_max, v)

ansmax = cnt+curr_max


curr_min = 0
curr_max = 0
curr = 0
for a in al:
    if a == 0:
        a -= 1
    curr += a
    curr_max = max(curr, curr_max)
    v = curr - curr_max
    curr_min = min(curr_min, v)

ansmin = cnt+curr_min

print(ansmax-ansmin+1)
