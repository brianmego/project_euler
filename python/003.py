target_num = 600851475143
test_val = 3
prime_list = [2]
while test_val < (target_num ** (1/2)):
    is_prime = True
    for i in prime_list:
        if test_val % i == 0:
            is_prime = False
            test_val += 2
    if is_prime:
        prime_list.append(test_val)
        if target_num % test_val == 0:
            target_num = int(target_num / test_val)
print(target_num)
