from re import S
import string

alphabet = list(string.ascii_uppercase)

ss = [[c] for c in alphabet]
while 1:
    s = ss.pop(0)
    if len(s) == 3:
        break

    for c in alphabet:
        sd = s
        ss.append(sd + [c])


for s in ss:
    if "".join(s) <= "ABC":
        print("".join(s))
