

def main():
    for test_i in range(int(input())):
        s = input()
        n = len(s)
        ans = []
        for i in range(n-1):
            for j in range(i+1, n):
                if s[i] < s[j]:
                    ans.append(s[i])
                    ans.append(s[i])
                    break
                elif s[i] > s[j]:
                    ans.append(s[i])
                    break
            else:
                ans.append(s[i])

        # if n != 1:
        ans.append(s[-1])
        print('Case #{}: {}'.format(test_i+1, ''.join(ans)))


if __name__ == "__main__":
    main()
