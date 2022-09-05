s = input("s > ").rstrip()

def conv(ci, c1, c2):
    if ci == c1:
        return c2
    if ci == c2:
        return c1
    return ci

while 1:
    c1, c2 = input().rstrip().split(" ")
    s = "".join(conv(ci, c1, c2) for ci in s)
    print(s)

