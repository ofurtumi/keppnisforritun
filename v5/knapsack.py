# import sys

# s = []
# for i in sys.stdin:
#     temp = list(map(lambda c: int(c), i.split()))
#     s.append(temp)

# s = [[5, 3], [1, 5], [10, 5], [100, 5], [6, 4], [5, 4], [4, 3], [3, 2], [2, 1]]
s = [[6, 4], [5, 4], [4, 3], [3, 2], [2, 1]]
# [value, weight]





mem = {}


def B(i, t):
    if t < 0:
        return -99999999
    if i < 0:
        return 0

    if i not in mem:
        mem[i] = max(s[i][0] + B(i+1, t - s[i][1], n), B(i+1, t, n))
        print(f"{i-1}, value: {s[i][0]}, weight: {s[i][1]}")
    return mem[i]

count = 0
while count < len(s):
    mem = {}
    print(f"B({1+count}, {s[count][0]}, {s[count][1]}) = ",
          B(1+count, s[count][0], s[count][1]))
    count += s[count][1]+1
    print(mem)
