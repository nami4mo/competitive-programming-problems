n = int(input())
al = list(map(int, input().split()))

for i in range(2111):
    if i not in al:
        print(i)
        break
