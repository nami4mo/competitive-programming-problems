

def main():
    n = int(input())
    al = list(map(int, input().split()))
    MAX = 2*10**5+1
    # MAX = 10
    cnts = [0]*MAX

    for a in al:
        cnts[a] += 1
    exist_cnt = 0
    for i in range(MAX):
        if cnts[i] > 0:
            exist_cnt += 1
    # print(cnts)

    ans = 0
    for i in range(MAX):
        c = cnts[i]
        v = c*(c-1)*(c-2)//6
        ans += v

    # print(ans)

    for i in range(MAX):
        c = cnts[i]
        v = c*(c-1)//2
        v *= n-c
        ans += v

    aall = n*(n-1)*(n-2)//6
    ans = aall-ans
    print(ans)


if __name__ == "__main__":
    main()
