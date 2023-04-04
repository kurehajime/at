n = int(input())
humans = list(input())
prev = 'x'
for h in humans:
    if prev == h:
        print('No')
        exit()
    prev = h
print('Yes')
