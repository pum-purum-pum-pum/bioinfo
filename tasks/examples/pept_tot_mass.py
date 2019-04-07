import numpy as np
ms = [57,71,87,97,99,101,103,113,114,115,128,129,131,137,147,156,163,186]
m = 1024
combinations = np.zeros(m + 1)
combinations[0] = 1
for i in range(m + 1):
    for j in ms:
        if i >= j:
            combinations[i] += combinations[i - j]
print(combinations[m])