

from itertools import permutations
import random


def main():
    # n = int(input())
    # l = [1]*n + [-1]*n
    # random.shuffle(l)

    # inp = input()
    # l = []
    # for v in inp:
    #     if v == '(':
    #         l.append(1)
    #     else:
    #         l.append(-1)
    # n = len(l)//2

    n = int(input())
    l = [1]*n + [-1]*n
    kakko_p = list(permutations(l, 2*n))
    st = set()
    kakko_l = []
    for kp in kakko_p:
        hs = map(str, kp)
        hs = ''.join(hs)
        if hs in st:
            continue
        st.add(hs)
        kakko_l.append(kp)

    res = []
    for l in kakko_l:
        lis = list(range(2*n))
        perml = list(permutations(lis, 2*n))
        ans = []
        for perm in perml:
            kakko = []
            for p in perm:
                kakko.append(l[p])
            curr = 0
            for v in kakko:
                curr += v
                if curr < 0:
                    break
            else:
                ans.append(perm)
        ans.sort()
        s = []
        for v in l:
            if v == 1:
                s.append('(')
            else:
                s.append(')')
        s = ''.join(s)
        # print(s, ans[0], ans[-1])
        res.append((s, ans[0], ans[-1]))

    res.sort(key=lambda x: x[1])
    for r in res:
        print(r[0], r[1], r[2])


if __name__ == "__main__":
    main()
