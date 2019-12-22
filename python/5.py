#!/usr/bin/env python
step_size = 20
target = step_size
numbers_to_attempt = range(target, 0, -1)

while True:
    success = True
    for divisor in numbers_to_attempt:
        if target % divisor != 0:
            success = False
            break
    if success:
        print(target)
        break
    target += step_size
