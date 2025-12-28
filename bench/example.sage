import time

start = time.time()

total = NN(0)
for x in range(1001):
    total = total + NN(x).factorial()

print(total)
end = time.time()
print(f"Elapsed time: {end - start:.6f} seconds")

