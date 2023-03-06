n, k = [int(x) for x in input().split()]
characters = [int(input(), 2) for i in range(n)]


distance = [21 for i in range(2**k)]
for character in characters:
    distance[character] = 0


top = 0
while (len(characters) > top):
    x = characters[top]
    top += 1
    for i in range(k):
        y = x ^ (1 << i)
        if (distance[x]+1 < distance[y]):
            characters.append(y)
            distance[y] = distance[x] + 1

print(distance)
print((bin(distance.index(max(distance)))[2:]).zfill(k))
