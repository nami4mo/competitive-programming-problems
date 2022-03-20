n = int(input())
al = list(map(int, input().split()))
bl = list(map(int, input().split()))

ans1 = 0
for i in range(n):
    if al[i] == bl[i]:
        ans1 += 1

ans2 = 0
for i in range(n):
    a = al[i]
    for j in range(n):
        if a == bl[j] and i != j:
            ans2 += 1
print(ans1)
print(ans2)
