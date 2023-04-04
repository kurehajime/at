[n, x] = list(map(int, input().split(" ")))
arr = input().split(" ")
hit = set()
for i in range(0, n):
    num = int(arr[i])
    prev = num - x
    next = num + x
    hit.add(prev)
    hit.add(next)
    if num in hit:
        print("Yes")
        exit()
print("No")
