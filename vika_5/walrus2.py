values = []
n = int(input())

for i in range(n):
    values.append(int(input()))

results = [False for i in range(2001)]
results[0] = True


def lift():
    for i in range(len(values)):
        val = values[i]
        from_value = 2001 - val
        for j in reversed(range(from_value)):
            if results[j]:
                results[j + val] = True


lift()

for i in range(len(results)):
    if results[1000 + i]:
        print(1000 + i)
        break
    elif results[1000 - i]:
        print(1000 - i)
        break
