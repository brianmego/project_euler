target_num = 600851475143
test_val = 2
while test_val < (target_num ** (.5)):
    while target_num % test_val == 0:
        target_num = target_num / test_val
    test_val += 1
print(int(target_num))
