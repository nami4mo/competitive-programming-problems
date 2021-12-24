from math import ceil

for x in range(-10000,10000):
    for n in range(1,200):

        m1 = ceil((-x+1)/n)
        m2 = -x//n +1

        if m1!=m2: print(m1)