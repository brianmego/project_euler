#!/usr/bin/env python
a = 1
b = 1
total = 0
while b < 4000000:
    c = b
    b = a + b
    a = c
    if b % 2 == 0:
        total += b
print(total)
