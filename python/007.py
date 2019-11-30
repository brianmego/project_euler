primes = [2]
target_index = 10001

current = 3
while len(primes) < target_index:
    is_prime = True
    for i in primes:
        if current % i == 0:
            is_prime = False
            break
    if is_prime:
        primes.append(current)
    current += 2

print(primes[-1])
