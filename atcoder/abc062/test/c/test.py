import random
import subprocess


c = 1
while 1:
    h = random.randint(2, 100)
    w = random.randint(2, 100)
    with open(f"in/c/{c}", "w") as f:
        print(h, w, file=f)
    raw_1 = subprocess.check_output(f"python3 test/c/c_lazy.py < in/c/{c}", shell=True)
    raw_2 = subprocess.check_output(f"cargo run --bin c --release < in/c/{c}", shell=True)
    result_1 = int(raw_1.decode().rstrip())
    result_2 = int(raw_2.decode().rstrip())
    print(result_1, result_2)
    if result_1 != result_2:
        print(f"Check in/c/{c}")
        break
    c += 1
