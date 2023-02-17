input = input().strip()
[n, x, y] = map(int, input.split(" "))
red = [0 for j in range(n+1)]
blue = [0 for j in range(n+1)]
red[n] = 1


def crush_red(i):
    if i <= 1:
        return
    red[i-1] += red[i]
    blue[i] += red[i] * x
    red[i] = 0


def crush_blue(i):
    if i <= 1:
        return
    red[i-1] += blue[i]
    blue[i-1] += blue[i] * y
    blue[i] = 0


for i in range(n, 0, -1):
    crush_red(i)
    crush_blue(i)

print(blue[1])
