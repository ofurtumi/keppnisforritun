n = input()
nums = input().split(" ")

for i, num in enumerate(nums):
    if num != "mumble" and int(num) != i+1:
        print("something is fishy")
        exit()

print("makes sense")