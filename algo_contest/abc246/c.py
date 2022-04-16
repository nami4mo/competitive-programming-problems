n, k, x = map(int, input().split())
al = list(map(int, input().split()))

rem = k
for i in range(n):
    a = al[i]
    cnt = a//x
    if cnt <= rem:
        rem -= cnt
        al[i] -= cnt*x
    else:
        al[i] -= rem*x
        rem = 0


al.sort(reverse=True)

for i in range(min(rem, n)):
    al[i] = 0


print(sum(al))
