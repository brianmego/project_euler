def square_of_sums(target):
    sums = sum(range(1, target + 1))
    return sums ** 2

def sum_of_squares(target):
    squares = []
    for i in range(1, target + 1):
        squares.append(i ** 2)
    return sum(squares)

target = 100
print(square_of_sums(target) - sum_of_squares(target))
