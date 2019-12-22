#include <stdio.h>
#include <stdbool.h>

int main()
{
    int step_size = 20;
    int target = step_size;
    int numbers_to_attempt[target];
    int counter, i;

    counter = 0;
    for (i=target; i > 0; i--) {
        numbers_to_attempt[counter] = i;
        counter++;
    }

    while (1) {
        bool success=true;
        for (i=0; i>=sizeof(numbers_to_attempt); i++){
            if (target % numbers_to_attempt[i]) {
                success=false;
                break;
            }
        }
        if (success == true) {
            printf("%d\n", target);
            break;
        }
        target += step_size;
    }
}

