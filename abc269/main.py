import re
n = int(input())
b = bin(n)[2:]
bx = b.replace("1", "x")
all_one_s = b.replace("0", "")
all_one = int(all_one_s, 2) if len(all_one_s) > 0 else 0
for i in range(0, all_one+1):
    param = i
    param_b = ("{:0>" + str(len(all_one_s))+"}").format(bin(param)[2:])
    text = bx
    for j in range(0, len(param_b)):
        text = re.sub(r"x", param_b[j], text, 1)
    print(int(text, 2))
