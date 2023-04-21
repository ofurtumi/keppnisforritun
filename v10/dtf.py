
import math

def is_prime(n):
    if n <= 1:
        return False
    for i in range(2, int(math.sqrt(n)) + 1):
        if n % i == 0:
            return False
    return True

def has_six_divisors(n):
    count = 0
    for i in range(1, int(math.sqrt(n)) + 1):
        if n % i == 0:
            if n // i == i:
                count += 1
            else:
                count += 2
    return count == 3

max_num = 1000
p1 = 169
if not is_prime(p1):
    print("asdf")
    exit()
for p2 in range(p1 + 1, int(math.sqrt(max_num / p1)) + 1):
    if not is_prime(p2):
        continue
    n = p1 * p1 * p2
    if n <= max_num and has_six_divisors(n):
        print(n)

