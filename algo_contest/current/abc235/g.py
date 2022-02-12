def main():
    n = input()
    m = int(input())
    cl = list(map(int, input().split()))
    MOD = 998244353
    nlen = len(n)
    ans = 0

    bitn = 1 << 10

    dp_val_small = [[0]*(bitn) for _ in range(nlen+1)]
    dp_val_same = [[0]*(bitn) for _ in range(nlen+1)]

    dp_small = [[0]*(bitn) for _ in range(nlen+1)]
    dp_same = [[0]*(bitn) for _ in range(nlen+1)]
    dp_same[0][0] = 1

    p10s = [1]
    for i in range(nlen):
        val = p10s[-1]*10
        val %= MOD
        p10s.append(val)

    for i in range(nlen):
        ni = int(n[i])
        p10 = p10s[nlen-1-i]
        for num in range(10):
            bitv = 1 << num
            flag = num in cl
            for bit in range(bitn):
                new_bit = bit | bitv
                if not flag:
                    new_bit = bit

                if bit <= 1 and num == 0:
                    continue
                if num < ni:
                    dp_small[i+1][new_bit] += dp_small[i][bit]
                    dp_small[i+1][new_bit] += dp_same[i][bit]
                    dp_val_small[i + 1][new_bit] += (dp_val_small[i][bit] * dp_small[i][bit])
                    dp_val_small[i + 1][new_bit] += (dp_val_same[i][bit] * dp_same[i][bit])
                    dp_val_small[i + 1][new_bit] += num*p10*dp_small[i][bit]
                    dp_val_small[i + 1][new_bit] += num*p10*dp_same[i][bit]

                elif num == ni:
                    dp_small[i+1][new_bit] += dp_small[i][bit]
                    dp_same[i+1][new_bit] += dp_same[i][bit]
                    dp_val_small[i + 1][new_bit] += (dp_val_small[i][bit] * dp_small[i][bit])
                    dp_val_same[i + 1][new_bit] += (dp_val_same[i][bit] * dp_same[i][bit])
                    dp_val_small[i + 1][new_bit] += num*p10*dp_small[i][bit]
                    dp_val_same[i + 1][new_bit] += num*p10*dp_same[i][bit]
                else:
                    dp_small[i+1][new_bit] += dp_small[i][bit]
                    dp_val_small[i + 1][new_bit] += (dp_val_small[i][bit] * dp_small[i][bit])
                    dp_val_small[i + 1][new_bit] += num*p10*dp_small[i][bit]

                if dp_small[i+1][new_bit] > MOD:
                    dp_small[i+1][new_bit] -= MOD
                if dp_small[i+1][new_bit] > MOD:
                    dp_small[i+1][new_bit] -= MOD
                dp_val_same[i+1][new_bit] %= MOD
                dp_val_small[i+1][new_bit] %= MOD
    want = 0
    for c in cl:
        want |= (1 << c)

    ans = 0
    for bit in range(bitn):
        if bit & want == want:
            # if bit == want:
            ans += dp_val_small[nlen][bit]+dp_val_same[nlen][bit]
            ans %= MOD
    print(ans)
    # print(dp_val_small[nlen])


if __name__ == '__main__':
    main()
