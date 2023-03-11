n = int(input())
polls = list(map(int, input().split()))

points = [0 for i in polls]

for i in range(1, len(polls)):
    if i == 1:
        points[i] = abs(polls[i-1] - polls[i])
        continue
    diff_1 = abs(polls[i-1] - polls[i])
    diff_2 = abs(polls[i-2] - polls[i])
    point_1 = diff_1 + points[i-1]
    point_2 = diff_2 + points[i-2]

    if point_1 < point_2:
        points[i] = point_1
    else:
        points[i] = point_2

print(points[len(points)-1])
