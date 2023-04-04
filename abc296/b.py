s = ""
x = 0
y = 0
label = ["a", "b", "c", "d", "e", "f", "g", "h"]
for _ in range(0, 8):
    s += input()
for i in range(0, len(s)):
    if s[i] == "*":
        x = i % 8
        y = i // 8
        break
result = label[x] + str(8-y)
print(result)
