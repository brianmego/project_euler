#include <stdio.h>
#include <stdbool.h>

int get_prime(int);

int main()
{
    static int target_index = 10001;
    printf("%d\n", get_prime(target_index));
}

int get_prime(int target_index) {
    int primes[target_index];
    int current=3;
    int j;
    int prime_index = 1;
    bool is_prime;

    primes[0] = 2;
    while (prime_index < target_index) {
        is_prime = true;
        for (j=0; j< prime_index; j++) {
            if (current % primes[j] == 0) {
                is_prime = false;
                break;
            }
        }
        if (is_prime) {
            primes[prime_index] = current;
            prime_index++;
        }
        current += 2;
    }
    return primes[target_index - 1];
}
