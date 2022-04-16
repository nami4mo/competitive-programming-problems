
from itertools import product


def base10int(value, base):
    if (int(value / base)):
        return base10int(int(value / base), base) + str(value % base)
    return str(value % base)


def i2s(n):
    return base10int(n, 4)


def s2i(s):
    res = 0
    for i in range(len(s)):
        res += 4**i * int(s[i])
    return res


def main():
    vals = set()
    vs = range(4**9)
    for pattern in vs:
        s = i2s(pattern)
        val = []
        cnts = [0]*4
        for v in s:
            v = int(v)
            if v == 0:
                continue
            cnts[v] += 1
            if cnts[v] > 3:
                break
            val.append(str(v))
        else:
            vals.add(s2i(val))
    vals = list(vals)
    vals.sort()
    print(len(vals))

    mp = {}

    for v1 in vals:
        mp[v1] = {}
        sv1 = i2s(v1)
        # print(v1)
        for v2 in vals:
            sv2 = i2s(v2)
            rem = 0
            for i in range(min(len(sv1), len(sv2))):
                if sv1[i] == sv2[i]:
                    rem = i+1
                else:
                    break
            need = len(sv1)+len(sv2)-rem*2
            mp[v1][v2] = need
    print(len(mp))

    for case_i in range(int(input())):
        n, w = map(int, input().split())
        last_len = 0
        dp = [{} for _ in range(n+1)]
        dp[0][0] = 0
        for i in range(n):
            al = list(map(int, input().split()))
            last_len = sum(al)
            # for i in range(w):
            #     al[i] -= 1
            for v1, cnt in dp[i].items():
                for v2 in vals:
                    cnts = [0]*w
                    for vi in i2s(v2):
                        if int(vi)-1 >= w:
                            break
                        cnts[int(vi)-1] += 1
                    else:
                        print(i2s(v2))

                        if al != cnts:
                            continue
                        if v2 in dp[i+1]:
                            dp[i+1][v2] = min(cnt+mp[v1][v2], dp[i+1][v2])
                        else:
                            dp[i+1][v2] = cnt+mp[v1][v2]
            print(dp)
        ans = min(dp[n].values())
        ans += last_len
        print('Case #{}: {}'.format(case_i+1, ans))

    # print(dp)


if __name__ == "__main__":
    main()
