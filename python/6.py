#!/usr/bin/env python
def main(target):
    print(square_of_sums(target) - sum_of_squares(target))

def square_of_sums(target):
    sums = sum(range(1, target + 1))
    return sums ** 2

def sum_of_squares(target):
    squares = []
    for i in range(1, target + 1):
        squares.append(i ** 2)
    return sum(squares)

if __name__ == '__main__':
    main(100)
