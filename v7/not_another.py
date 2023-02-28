def find(data, i):
    if i != data[i]:
        data[i] = find(data, data[i])
    return data[i]


def union(data, i, j):
    pi, pj = find(data, i), find(data, j)
    if pi != pj:
        data[pi] = pj


def connected(data, i, j):
    return find(data, i) == find(data, j)


n = input()

data = list(map(int, input().split(" ")))

# uf = UnionFind(data)

print(data)
union(data, 1, 2)
print(data)

# for i in range(int(input())):
#     (i, j) = map(int, input().split(" "))
#     temp = uf._siz[uf.find(i)]
#     uf.union(i, j)
#     print(temp, uf._siz[uf.find(j)])
