import math

(num_purchase, num_enemies) = map(int, input().split())
(hp, damage) = map(int, input().split())
purchases = list(map(int, input().split()))
purchases.sort()
enemies = list(map(int, input().split()))


def main():

    if sum([math.ceil(enemy/damage)-1 for enemy in enemies]) < hp:
        print("Veski Eyleifs er bjargad")
        return

    if sum([math.ceil(enemy/(damage+purchases[num_purchase-1]))-1 for enemy in enemies]) >= hp:
        print("Nu er Eyleifur i bobba")
        return

    print(div(0, len(purchases)-1))


def div(i, j):
    if j - i <= 1:
        dat = []
        for x in range(i-2, i+3):
            if x >= 0 and x < len(purchases):
                dat.append(
                    [x, sum([math.ceil(enemy/(damage+purchases[x]))-1 for enemy in enemies])])
        for x in range(len(dat)):
            if dat[x][1] < hp:
                return purchases[dat[x][0]]
    dat = sum(
        [math.ceil(enemy/(damage+purchases[i+((j-i)//2)]))-1 for enemy in enemies])
    if dat < hp:
        return div(i, i+((j-i)//2))
    else:
        return div(i+((j-i)//2), j)

if __name__ == "__main__": main()