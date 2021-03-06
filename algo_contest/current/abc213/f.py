def Z_algorithm(s):
    N = len(s)
    Z_alg = [0]*N

    Z_alg[0] = N
    i = 1
    j = 0
    while i < N:
        while i+j < N and s[j] == s[i+j]:
            j += 1
        Z_alg[i] = j
        if j == 0:
            i += 1
            continue
        k = 1
        while i+k < N and k + Z_alg[k]<j:
            Z_alg[i+k] = Z_alg[k]
            k += 1
        i += k
        j -= k
    return Z_alg

n=int(input())
s=input()
zet=Z_algorithm(s)

alps = 'abcdefghijklmnopqrstuvwxyz'  # string.ascii_lowercase
# alps = [chr(ord('a')+i) for i in range(26)]
alps=[[] for _ in range(26)]

ans=0
for z in zet:
    ans+=z

