import sys
input = sys.stdin.readline


def main():
    for _ in range(int(input())):
        n = int(input())
        al = list(map(int, input().split()))
        if n % 2 == 1:
            print('Mike')
        else:
            bl = []
            cl = []
            mi = min(al)
            for i in range(n):
                if i % 2 == 0:
                    bl.append(al[i]-mi)
                else:
                    cl.append(al[i]-mi)
            for i in range(n):
                if i % 2 == 0:
                    if bl[i//2] == 0:
                        print('Joe')
                        break
                else:
                    if cl[i//2] == 0:
                        print('Mike')
                        break
            # if min(bl) <= min(cl):
            #     print('Joe')
            # else:
            #     print('Mike')


if __name__ == "__main__":
    main()
