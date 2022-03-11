import sys
import numpy as np

read = sys.stdin.buffer.read
readline = sys.stdin.buffer.readline
readlines = sys.stdin.buffer.readlines

Bsize = 700


def Mo_argsort(LR):
    L = LR[:, 0]
    R = LR[:, 1]
    key1 = L // Bsize
    key2 = np.where(key1 & 1, -R, R)
    key = (key1 << 32) + key2
    return np.argsort(key)


def main(A, LR):
    N = len(A)
    Q = len(LR)
    ind = Mo_argsort(LR)
    count = np.zeros(N + 1, np.int64)
    ans = 0

    def add(x):
        nonlocal ans
        if not count[x] % 2 == 1:
            ans += 1
        count[x] += 1

    def rem(x):
        nonlocal ans
        count[x] -= 1
        if not count[x] % 2 == 1:
            ans -= 1

    answers = np.empty(Q, np.int64)
    nl, nr = 0, 0
    for i in ind:
        l, r = LR[i]
        l -= 1
        # 区間 [l, r) に対する計算
        while nl > l:
            nl -= 1
            add(A[nl])
        while nr < r:
            add(A[nr])
            nr += 1
        while nl < l:
            rem(A[nl])
            nl += 1
        while nr > r:
            nr -= 1
            rem(A[nr])
        #  答を求めて ans[i] に格納する
        answers[i] = ans
    return answers


N = int(input())
A = list(map(int, input().split()))
Q = int(input())
LR = []
for _ in range(Q):
    l, r = map(int, input().split())
    LR.append([l, r])
LR = np.array(LR)

ans = main(A, LR)
print('\n'.join(map(str, ans.tolist())))
