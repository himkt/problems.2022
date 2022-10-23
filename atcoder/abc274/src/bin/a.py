a, b = list(map(int, input().rstrip().split()))
c = b / a
c = round(c, 3)
c = c % 10.0
print(f"{c:0.3f}")
