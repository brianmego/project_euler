#!/usr/bin/env python


def main():
    longest_start = 1
    longest_count = 1
    cached = {}
    steps = 0
    for i in range(1, 1000000):
        cur = i
        count = 1
        while cur != 1:
            steps += 1
            if cur in cached:
                to_add = cached[cur]
                count += to_add - 1
                break
            if cur % 2 == 0:
                cur = cur / 2
            else:
                cur = (cur * 3) + 1
            count += 1
        cached[i] = count
        if count > longest_count:
            longest_start = i
            longest_count = count

    # print(str(sum(INPUTS))[:10])
    print(f'Start Number: {longest_start}; Count: {longest_count}; Steps: {steps}')

if __name__ == "__main__":
    main()

