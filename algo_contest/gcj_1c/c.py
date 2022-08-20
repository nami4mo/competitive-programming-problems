
import random
import sys
input = sys.stdin.readline


def popcount(n):
    return bin(n).count("1")


def tob(n):
    return format(n, '08b')
    # return bin(n)[2:]


def main():
    rd = [7, 14, 28, 56, 112, 224, 193, 131]
    # rd = [7, 31]
    for _case in range(int(input())):
        prev_pc = 4
        use_all = False
        for _ in range(300):
            if use_all:
                val = 255
            else:
                # val = (1 << (8-prev_pc))-1
                # print(val, file=sys.stderr)
                val = random.choice(rd)
                # val = 7
            print(tob(val), flush=True)
            pc = int(input())
            if pc == 0 or pc == -1:
                break
            if pc == 8:
                use_all = True
            prev_pc = pc


if __name__ == "__main__":
    main()
