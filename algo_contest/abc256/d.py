

def main():
    n = int(input())
    lrs = []
    for _ in range(n):
        l, r = map(int, input().split())
        lrs.append((l, r))
    lrs.sort()
    curr_l = lrs[0][0]
    curr_r = lrs[0][1]
    ans = []
    if n == 1:
        print(curr_l, curr_r)
        return
    for l, r in lrs[1:]:
        if curr_l <= l <= curr_r:
            curr_r = max(curr_r, r)
        else:
            ans.append((curr_l, curr_r))
            curr_l = l
            curr_r = r
    if (not ans) or ans[-1] != (curr_l, curr_r):
        ans.append((curr_l, curr_r))

    for l, r in ans:
        print(l, r)


if __name__ == "__main__":
    main()
