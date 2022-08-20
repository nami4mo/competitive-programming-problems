n = int(input())

al = [[0]*n for _ in range(n)]
for i in range(n):
    for j in range(i+1):
        if j == 0 or j == i:
            al[i][j] = 1
        else:
            al[i][j] = al[i-1][j-1]+al[i-1][j]


for i in range(n):
    print(*al[i][:i+1])
