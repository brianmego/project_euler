#!/usr/bin/env python
LARGEST = 999
SMALLEST = 100

def is_palindrome(i):
    return str(i) == str(i)[::-1]

def has_three_digit_factors(i):
    for j in range(LARGEST, SMALLEST - 1, -1):
        if i % j == 0:
            quotient = i / j
            if SMALLEST <= quotient <= LARGEST:
                return True
    return False

def main():
    for i in range(LARGEST**2, SMALLEST, -1):
        if is_palindrome(i):
            if has_three_digit_factors(i):
                print(i)
                break

if __name__ == '__main__':
    main()
