

def main():
    x = input()
    up = 0
    csum = 0
    ans = []
    for xi in x:
        csum += int(xi)
    ans.append(str(csum % 10))
    up = csum//10
    for xi in x[::-1]:
        csum -= int(xi)
        val = up+csum
        ans.append(str(val % 10))
        up = val//10
    ans = ''.join(ans[::-1])
    if ans[0] == '0':
        print(ans[1:])
    else:
        print(ans)


if __name__ == "__main__":
    main()
