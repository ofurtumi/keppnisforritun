n = int(input())
e = list(map(int, input().split(' ')))
e.sort()

sum = 0
for i in range(n-1):
    sum += (e[i] - e[i+1])**2

print(sum)