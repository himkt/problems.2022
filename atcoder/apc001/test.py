import random


NL = random.randint(1, 10)
NR = random.randint(1, 10)

vacant_idx = 2 * NL
N = 2 * NL + 2 * NR + 1

seed_list = ["Male", "Female"]
random.shuffle(seed_list)

list_l = seed_list * NL
list_r = seed_list * NR

list = list_l + ["Vacant"] + list_r
print(f"list: {list}")
print(f"n: {N}, ground truth: {vacant_idx}")

while 1:
    query = int(input())
    print(list[query])
