from bisect import bisect_left
import sys

arr = []
for i in sys.stdin:
    temp = list(map(lambda c: int(c), i.split()))
    arr.append(temp)

arr = [i[0] for i in arr]

n = arr.pop(0)

mem = []


def sub_sums(arr, i, n, sum=0):
    if i > n:
        mem.append(sum)
        return
    sub_sums(arr, i+1, n, sum + arr[i])
    sub_sums(arr, i+1, n, sum)


sub_sums(arr, 0, len(arr)-1)
mem.sort()


def take_closest(myList, myNumber):
    pos = bisect_left(myList, myNumber)
    if pos == 0:
        return myList[0]
    if pos == len(myList):
        return myList[-1]
    before = myList[pos - 1]
    after = myList[pos]
    if after - myNumber < myNumber - before:
        return before
    else:
        return after


choice = take_closest(mem, 1000)
print(choice)
