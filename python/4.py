#!/usr/bin/env python
LARGEST = 999

def is_palindrome(i):
    return str(i) == str(i)[::-1]

def has_three_digit_factors(i):
    for j in range(999, 500, -1):
        if i % j == 0:
            quotient = i / j
            if quotient > 100 and quotient <= 999:
                return True

for i in range(LARGEST**2, 100, -1):
    if is_palindrome(i):
        if has_three_digit_factors(i):
            print(i)
            break
