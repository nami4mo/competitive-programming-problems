

def main():
    n, m = map(int, input().split())
    swap = False
    if n > m:
        n, m = m, n
        swap = True
    ans = []
    curr_x = 1
    st = set()
    for v in range(4, 3*n+m+1):
        next_x = curr_x+1
        done = False
        if next_x*3+1 <= v and curr_x < n:
            y = v-3*next_x
            vv = next_x+3*y
            if vv not in st and y <= m:
                curr_x += 1
                ans.append((curr_x, y, 0))
                st.add(vv)
                done = True
        if not done:
            y = v-3*curr_x
            if y <= m:
                ans.append((curr_x, y, 1))
                st.add(curr_x+3*y)

    print(len(ans))
    for i, j, k in ans:
        if swap:
            i, j = j, i
        print(i, j)


if __name__ == "__main__":
    main()
