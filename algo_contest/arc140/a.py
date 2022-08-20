

def main():
    n, k = map(int, input().split())
    s = list(input())
    ans = 10**18

    if n <= k:
        print(1)
        return
    divs = []
    for i in range(1, int(n**0.5)+1):
        if n % i == 0:
            divs.append(i)
            if i*i != n:
                divs.append(n//i)

    divs.sort()
    for d in divs:
        dics = [{} for _ in range(d)]
        for j in range(n):
            v = j % d
            dics[v].setdefault(s[j], 0)
            dics[v][s[j]] += 1
        cnt = 0
        for i in range(d):
            vals = list(dics[i].values())
            vals.sort()
            cnt += n//d-vals[-1]
        if cnt <= k:
            print(d)
            return


if __name__ == "__main__":
    main()
