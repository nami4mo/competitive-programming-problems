

def main():
    w = int(input())
    ans = []
    for i in range(1, 100):
        ans.append(i)
        ans.append(i*10**2)
        ans.append(i*10**4)
    print(len(ans))
    print(*ans)

    # al = ans[:]
    # st = set()
    # n = len(ans)
    # w = 10**6
    # for i in range(n):
    #     if al[i] <= w:
    #         st.add(al[i])
    #     for j in range(i+1, n):
    #         if al[i]+al[j] <= w:
    #             st.add(al[i]+al[j])
    #         for k in range(j+1, n):
    #             if al[i]+al[j]+al[k] <= w:
    #                 st.add(al[i]+al[j]+al[k])
    # print(st)
    # print(len(st))


if __name__ == "__main__":
    main()
