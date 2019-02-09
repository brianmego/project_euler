primes = []
max_num = 600851475143
for i in range(2, max_num):
    prime = True
    for j in primes:
        if i % j == 0:
            prime = False
            break
    if prime:
        primes.append(i)
        print(i)
