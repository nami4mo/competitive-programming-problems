

def main():
    n = int(input())

    def rec(i):
        if i == 1:
            return [1]
        v = rec(i-1)
        return v + [i] + v

    res = rec(n)
    print(*res)


if __name__ == "__main__":
    main()
