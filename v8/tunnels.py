n = 6
t = 0
adj = [
    [],
    [2],
    [1, 3, 5],
    [2],
    [5],
    [2, 4, 6],
    [5]
]

visited = [False] * (n + 1)
tin = [-1] * (n + 1)
low = [-1] * (n + 1)


def IS_BRIDGE(v, to):
    print(f"{v} to {to} is a bridge")


def dfs(v, p):
    visited[v] = True
    global t
    t += 1

    tin[v] = t
    low[v] = t

    for to in adj[v]:
        if to == p:
            continue
        if visited[to]:
            low[v] = min(low[v], tin[to])
        else:
            dfs(to, v)
            low[v] = min(low[v], low[to])
            if low[to] > tin[v]:
                IS_BRIDGE(v, to)


for i in range(n):
    if not visited[i]:
        dfs(i, -1)
