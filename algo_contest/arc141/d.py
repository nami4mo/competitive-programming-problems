

def main():
    n = int(input())*2
    pl = list(map(int, input().split()))
    ql = list(map(int, input().split()))
    pl = [p-1 for p in pl]
    ql = [q-1 for q in ql]
    qpos = [-1]*n
    for i in range(n):
        qpos[ql[i]] = i

    ans = [0]*n
    close_l = -1
    curr = 0
    ng = False
    for p in pl:
        if curr > 0:  # どっちでもいい
            break


if __name__ == "__main__":
    main()
