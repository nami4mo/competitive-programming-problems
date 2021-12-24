import math

def get_circle_center_and_radius(x1, y1, x2, y2, x3, y3):
    d = 2 * ((y1 - y3) * (x1 - x2) - (y1 - y2) * (x1 - x3))
    if d == 0:
        x = (x1+x3)/2
        y = (y1+y3)/2
        r2 = (x3-x)**2 + (y3-y)**2
        # r = math.sqrt(r2)
        return (x,y), r2

    a = (x3-x2)**2 + (y3-y2)**2
    b = (x1-x2)**2 + (y1-y2)**2
    c = (x3-x1)**2 + (y3-y1)**2
    if a+b<c:
        x = (x3+x1)/2
        y = (y3+y1)/2
        r2 = c/4
    elif b+c<a:
        x = (x3+x2)/2
        y = (y3+y2)/2
        r2 = a/4
    elif c+a<b:
        x = (x1+x2)/2
        y = (y1+y2)/2
        r2 = b/4
    else:
        x = ((y1 - y3) * (y1 ** 2 - y2 ** 2 + x1 ** 2 - x2 ** 2) - (y1 - y2) * (y1 ** 2 - y3 ** 2 + x1 ** 2 - x3 ** 2)) / d
        y = ((x1 - x3) * (x1 ** 2 - x2 ** 2 + y1 ** 2 - y2 ** 2) - (x1 - x2) * (x1 ** 2 - x3 ** 2 + y1 ** 2 - y3 ** 2)) / -d
        # r = math.sqrt((x - x1) ** 2 + (y - y1) ** 2)
        r2 = (x - x1) ** 2 + (y - y1) ** 2
    return (x, y), r2

def main():
    n = int(input())
    xyl = []
    for _ in range(n):
        x,y = map(int, input().split())
        xyl.append((x,y))

    xyl.sort()
    if n == 2:
        x1,y1 = xyl[0]
        x2,y2 = xyl[1]
        r2 = (x1-x2)**2 + (y1-y2)**2
        r = math.sqrt(r2)
        print(r/2)
        exit()

    ans = 10**18
    for i in range(n):
        for j in range(i+1,n):
            for k in range(j+1,n):
                x1,y1 = xyl[i]
                x2,y2 = xyl[j]
                x3,y3 = xyl[k]
                (cx,cy),r2 = get_circle_center_and_radius(x1, y1, x2, y2, x3, y3)
                for x,y in xyl:
                    d2 = (x-cx)**2 + (y-cy)**2
                    if d2 <= r2+10**(-6):
                        pass
                    else:
                        break
                else:
                    ans = min(ans,r2)
                    
    print(math.sqrt(ans))

if __name__ == "__main__":
    main()