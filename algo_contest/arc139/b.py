import sys
input = sys.stdin.readline


def main():
    n, a, b, x, y, z = map(int, input().split())
    if a < b:
        a, b = b, a
        y, z = z, y

    if a*x <= y and b*x <= z:
        print(x)
    elif a*x <= y and b*x > z:
        bcnt = n//b
        ans = bcnt*z + (n - bcnt*b)*x
        print(ans)
    elif a*x > y and b*x <= z:
        acnt = n//a
        ans = acnt*y + (n - acnt*a)*x
        print(ans)
    else:
        if a >= 10**5:
            for acnt in range(10**5):
                if acnt*a > n:
                    break
                rem = n-acnt*a
                bcnt = rem//b
                val = acnt*y+bcnt*z+(rem-bcnt*b)*x
                ans = min(ans, val)
        else:
            maxi = b-1
            if a*z < b*y:  # aの方がコスパいいように
                a, b = b, a
                y, z = z, y
            for i in range(0, maxi+1):
                nn = n-i
                acnt = nn//a
                bcnt =


if __name__ == "__main__":
    for _ in range(int(input())):
        main()
