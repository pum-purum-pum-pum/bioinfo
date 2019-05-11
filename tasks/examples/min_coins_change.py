n = 16869
w = [1,3,5,6,7,10,11,15]
combinations = [1e16 for _ in range(n + 1)]
combinations[0] = 0
for i in range(n + 1):
    for j in range(len(w)):
        if i >= w[j]:
            combinations[i] = min(combinations[i], combinations[i - w[j]] + 1)
combinations[n]