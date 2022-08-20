import sys
input = sys.stdin.readline


def main():
    for _ in range(int(input())):
        s = input()
        al = []
        for si in s:
            v = ord(si)-ord('a')+1
            al.append(v)

        if len(al) % 2 == 0:
            ans = sum(al)
            print('Alice', ans)
        elif len(al) == 1:
            ans = sum(al)
            print('Bob', ans)
        else:
            ans = max(sum(al[1:])-al[0], sum(al[:-1])-al[-1])
            print('Alice', ans)


if __name__ == "__main__":
    main()
