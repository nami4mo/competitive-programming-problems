import sys
input = sys.stdin.readline


def main():
    MAX = 26
    for _ in range(int(input())):
        s = input().rstrip()
        n = len(s)
        ss = []
        for si in s:
            ss.append(ord(si)-ord('a'))
        s = ss[:]
        cs = set()
        for si in s:
            cs.add(si)
        cs = list(cs)
        csums = [[0]*MAX for _ in range(n+1)]
        for i in range(n):
            si = s[i]
            for j in range(MAX):
                if si == j:
                    csums[i+1][j] = csums[i][j]+1
                else:
                    csums[i+1][j] = csums[i][j]
        # print(csums)
        prevs = [-1]*MAX
        ok = True
        for i in range(n):
            si = s[i]
            if prevs[si] != -1:
                prev_i = prevs[si]
                for j in cs:
                    if j == si:
                        continue
                    cnt = csums[i][j]-csums[prev_i+1][j]
                    # print(i, prev_i)
                    if cnt == 0:
                        print('NO')
                        ok = False
                        break
            if not ok:
                break
            prevs[si] = i
        else:
            print('YES')


if __name__ == "__main__":
    main()
