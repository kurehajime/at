n = int(input())
arr = [input().split(' ') for i in range(n)]

used = set()
maxnum = -1
idx = -1
for i in range(0, len(arr)):
    st = arr[i]
    s = st[0]
    t = int(st[1])
    if s not in used:
        used.add(s)
        if maxnum < t:
            maxnum = t
            idx = i
print(idx + 1)
