#include <stdio.h>
#include <stdbool.h>

int main()
{
    static int desired_sum = 1000;
    int product = 0;
    int a, b, c, max_b;

    for (a=0; a < desired_sum; a++) {
        if (product != 0) {
            break;
        }
        max_b = (desired_sum - a) / 2;
        for (b=a; b < max_b; b++) {
            c = 1000 - a - b;

            if (a*a + b*b == c*c) {
                product = a * b * c;
                break;
            }
        }
    }
    printf("%d\n", product);
}
