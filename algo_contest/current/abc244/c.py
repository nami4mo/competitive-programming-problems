n = int(input())
rem = [True]*(2*n+2)
rem[0] = False

for i in range(n+1):
    for j in range(2*n+2):
        if rem[j]:
            print(j, flush=True)
            rem[j] = False
            break
    if i != n:
        v = int(input())
        rem[v] = False

print(0)
