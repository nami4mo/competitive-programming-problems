s = input()
ans = 0
ans += int(s[0]+s[1]+s[2])
ans += int(s[1]+s[2]+s[0])
ans += int(s[2]+s[0]+s[1])
print(ans)
