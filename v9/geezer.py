import heapq

A = 0
H = 0
n = 0
m = 0
hd = [0]*100002
zz = 0

e = [[0, 0, 0, 0]]*100002

def dijkstra():
    q = [(H,1)]
    heapq.heapify(q)

    while len(q) > 0:
        popped = heapq.heappop(q)
        health = popped[0]
        x = popped[1]

        if done[x]: continue

        for i in range(hd[x], )