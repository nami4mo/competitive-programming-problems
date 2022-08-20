n, w = map(int, input().split())

al = list(map(int, input().split()))

st = set()
for i in range(n):
    if al[i] <= w:
        st.add(al[i])
    for j in range(i+1, n):
        if al[i]+al[j] <= w:
            st.add(al[i]+al[j])
        for k in range(j+1, n):
            if al[i]+al[j]+al[k] <= w:
                st.add(al[i]+al[j]+al[k])

print(len(st))
