l, r, = map(int, input().split())
l -= 1
# r -= 1
s = input()
ss = s[0:l]+s[l:r][::-1]+s[r:]
print(ss)
