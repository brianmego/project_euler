#!/usr/bin/env python

desired_sum = 1000
product = 0

for a in range(desired_sum):
    if product:
        break
    max_b = int((desired_sum - a) / 2)
    for b in range(a, max_b):
        c = 1000 - a - b
        if a**2 + b**2 == c**2:
            product = a * b * c
            break
print(product)
