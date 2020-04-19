#include <stdio.h>
#include <math.h>
#include <stdbool.h>

int main()
{
    static long target = 2000000;
    long long total = 0;
    int primes[target];
    long i, j;
    for (i=0; i<= target; i++) {
        primes[i] = true;
    }

    for (i=2; i < sqrt(target); i++) {
        if (primes[i] == true) {
            for (j=i*2; j<target; j+=i) {
                primes[j] = false;
            }
        }
    }

    for (i=2; i < target; i++) {
        if (primes[i] == true) {
            total += i;
        }
    }

    printf("%lli\n", total);
}

