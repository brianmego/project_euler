#!/usr/bin/env python
from math import ceil

target = 2000000
total = 0
primes = [True] * target
primes[0] = False
primes[1] = False

for i in range(ceil(target ** .5)):
    if primes[i] is True:
        for j in range(i*2, target, i):
            primes[j] = False

for i, is_prime in enumerate(primes):
    if is_prime:
        total += i

print(total)
