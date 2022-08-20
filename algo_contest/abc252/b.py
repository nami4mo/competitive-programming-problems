
n, k = map(int, input().split())
al = list(map(int, input().split()))
bl = list(map(int, input().split()))

ma = max(al)
st = set()
for i in range(n):
    if al[i] == ma:
        st.add(i)

for b in bl:
    b -= 1
    if b in st:
        print('Yes')
        exit()
print('No')
