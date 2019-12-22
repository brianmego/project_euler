#include <stdio.h>
#include <stdbool.h>

int main()
{
    int step_size = 20;
    int target = step_size;
    int i;

    while (1) {
        bool success=true;
        for (i=1; i<=step_size; i++){
            if (target % i) {
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

