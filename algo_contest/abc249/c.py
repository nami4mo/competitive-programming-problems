

def main():
    n = int(input())
    al = list(map(int, input().split()))
    al.sort()
    MAX = 2*10**5+1
    # MAX = 10
    cnts = [0]*MAX
    for a in al:
        cnts[a] += 1
    ans = 0
    # print(al)
    for aj in range(1, MAX):
        cj = cnts[aj]
        for ai in range(aj, MAX, aj):
            ci = cnts[ai]
            # if aj == ai:
            #     ci -= 1
            ak = ai//aj
            ck = cnts[ak]
            # if aj == ak:
            #     ck -= 1
            # if ai == ak:
            #     ck -= 1
            cnt = cj*ci*ck

            # print(ai, aj, ak, "=", cnt)

            ans += cnt
    print(ans)


if __name__ == "__main__":
    main()
