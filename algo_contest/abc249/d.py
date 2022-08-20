

def main():
    n = int(input())
    al = list(map(int, input().split()))
    al.sort()
    # MAX = 2*10**5+1
    MAX = 10
    cnts = [0]*MAX
    for a in al:
        cnts[a] += 1
    ans = 0

    cnt1 = al.count(1)
    ans += cnt1*(cnt1-1)*(cnt1-2)

    print(cnts)
    for v in range(2, MAX):
        pair = cnts[v]*(cnts[v]-1)
        com = pair*cnt1
        ans += com

    print(ans)

    for v in range(2, MAX):
        if v*v > MAX:
            break
        if cnts[v] == 0:
            continue

        # com = cnts[v]*(cnts[v]-1)*cnts[v*v]
        # ans += com

        # start = v*(v+1)
        # print(start, v)
        start = v
        for v3 in range(start, MAX, v):
            v2 = v3//v
            # print(v3, v2, v)
            if v2 == v:
                com = cnts[v]*(cnts[v]-1)*cnts[v*v]
                ans += com
            else:
                com = cnts[v]*cnts[v2]*cnts[v3]
                ans += com
    print(ans)


if __name__ == "__main__":
    main()
