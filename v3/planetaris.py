import sys

s = []
for i in sys.stdin:
    temp = list(map(lambda c: int(c), i.split()))
    s.append(temp)

def attack(ships, fleet, points):
    if ships < 0:
        return -1
    elif len(fleet) == 0:
        return 0

    left = attack(ships, fleet[1:], points)
    right = attack(ships - (fleet[0] + 1), fleet[1:], points)

    if right >= left:
        return points + 1 + right
    return points + left

print(attack(s[0][1], s[1], 0))
