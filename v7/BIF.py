n, t = map(int, input().split())
works = []
for i in range(n):
    ti, fi = map(int, input().split())
    works.append((fi, ti)) # sort by burning time first, then fetch time
works.sort(reverse=True)

time_remaining = t
total_extension = 0
for i in range(n):
    fi, ti = works[i]
    if time_remaining <= ti:
        break
    extension = min(time_remaining - ti, fi)
    total_extension += extension
    time_remaining -= extension
    time_remaining += ti

print(t + total_extension)