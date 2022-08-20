n = int(input())
tl = list(map(int, input().split()))

st = []
for i in range(1, 42):
    st.append(pow(2, i))

nmi = 1
for t in tl:
    div = pow(2, t)
    if nmi <= div:
        nmi = div+1
    elif nmi % div == 0:
        while (nmi // div) % 2 == 0:
            nmi += div
        nmi = nmi+1
    else:
        rem = div-(nmi % div)
        v = nmi+rem
        while (v//div) % 2 == 0:
            v += div
        nmi = v+1
    # print(t, nmi-1, bin(nmi-1))
print(nmi-1)
