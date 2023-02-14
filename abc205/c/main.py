input = input().strip()
[a, b, c] = map(int, input.split(" "))
if c % 2 == 0 and abs(a) == abs(b):
    print("=")
elif c % 2 == 0 and abs(a) < abs(b):
    print("<")
elif c % 2 == 0 and abs(a) > abs(b):
    print(">")
elif c % 2 != 0 and a < b:
    print("<")
elif c % 2 != 0 and a > b:
    print(">")
else:
    print("=")
