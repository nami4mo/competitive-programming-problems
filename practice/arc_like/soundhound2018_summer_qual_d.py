import heapq
def dijkstra(s, n, g): # s: start, n: |V|, g; glaph 
    INF = 10**18
    d = [INF] * n
    d[s] = 0
    que = [] # (a,b): a... shortest dist, b... v
    heapq.heappush(que, (0, s))

    while que:
        dist, v = heapq.heappop(que)
        if d[v] < dist: continue # if v has been already used -> continue
        for next_v, cost in g[v]:
            if d[next_v] > d[v] + cost:
                d[next_v] = d[v] + cost
                heapq.heappush(que, (d[next_v], next_v))
    return d


n,m,s,t = map(int, input().split())
gal = [ [] for _ in range(n+1) ]
gbl = [ [] for _ in range(n+1) ]

for _ in range(m):
    u,v,a,b = map(int, input().split())
    gal[u].append((v,a))
    gal[v].append((u,a))
    gbl[u].append((v,b))
    gbl[v].append((u,b))

adl = dijkstra(s,n+1,gal)
bdl = dijkstra(t,n+1,gbl)
ansl = [0]*n
curr_min = 10**18

# print(adl)
for i in range(n-1,-1,-1):
    cost = adl[i+1] + bdl[i+1]
    curr_min = min(cost, curr_min)
    ansl[i] = 10**15-curr_min
    # print('--',i)
    # print(cost)

for ans in ansl:
    print(ans)
