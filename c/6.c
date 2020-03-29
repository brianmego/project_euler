#include <stdio.h>
#include <stdbool.h>

int square_of_sums(int);
int sum_of_squares(int);

int main()
{
    static int target = 100;
    printf("%d\n", (square_of_sums(target) - sum_of_squares(target)));
}

int square_of_sums(int target){
    int sum = 0;
    int i;
    for (i=1; i<=target; i++){
        sum += i;
    }
    return sum * sum;
}

int sum_of_squares(int target){
    int sum = 0;
    int i = 0;

    for (i=1; i<=target; i++){
        sum += i*i;
    }
    return sum;
}
