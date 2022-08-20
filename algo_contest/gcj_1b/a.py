
from collections import deque


def main():
    for case in range(int(input())):
        n = int(input())
        sl = map(str, input().split())
        new_sl = []
        for s in sl:
            new_sl.append(list(s))
        sl = new_sl[:]

        ssl = []
        nsl = []
        for s in sl:
            same = True
            for i in range(len(s)):
                if s[i] != s[0]:
                    same = False
            if same:
                ssl.append(s)
            else:
                nsl.append(s)
        sssl = []
        for s in ssl:
            for i in range(len(nsl)):
                if nsl[i][0] == s[-1]:
                    nsl[i] = s[:] + nsl[i][:]
                    break
                elif nsl[i][-1] == s[0]:
                    nsl[i] = nsl[i][:] + s[:]
                    break
            else:
                sssl.append(s)
        sl = nsl[:]
        same_sl = sssl[:]
        same_sl.sort(key=lambda x: x[0])
        same_sl_ans = []
        for s in same_sl:
            same_sl_ans.append(''.join(s))
        same_sl_ans = ''.join(same_sl_ans)
        # print('---')
        # print(sl)
        # print(same_sl)
        if len(sl) == 0:
            print('Case  #' + str(case+1) + ':', same_sl_ans)
            continue
        # elif len(sl) == 1:
        #     v = ''.join(sl[0])
        #     print('Case  #' + str(case+1) + ':', same_sl_ans + v)
        #     continue

        sl_ans = []
        used = [False]*len(sl)
        for i in range(len(sl)):
            if used[i]:
                continue
            q = deque()
            q.append(sl[i])
            used[i] = True

            while True:
                change = False
                for i in range(len(sl)):
                    if used[i]:
                        continue
                    if q[-1][-1] == sl[i][0]:
                        q.append(sl[i])
                        used[i] = True
                        change = True
                        break
                    elif q[0][0] == sl[i][-1]:
                        q.appendleft(sl[i])
                        used[i] = True
                        change = True
                        break
                if not change:
                    break
            # print(q, used)
            while q:
                poped = q.popleft()
                sl_ans.append(''.join(poped))
            # for i in range(len(sl)):
            #     if not used[i]:
            #         q.append(sl[i])

        sl_ans = ''.join(sl_ans)
        ans = same_sl_ans + sl_ans
        cntl = []
        prev = ans[0]
        cnt = 1
        for a in ans[1:]:
            if prev == a:
                cnt += 1
            else:
                cntl.append((prev, cnt))
                cnt = 1
                prev = a
        cntl.append((prev, cnt))
        st = set()
        for key, _ in cntl:
            if key in st:
                print('Case  #' + str(case+1) + ': IMPOSSIBLE')
                break
            st.add(key)
        else:
            print('Case  #' + str(case+1) + ':', ans)


if __name__ == "__main__":
    main()
