#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

bool isPalindrome(int);
bool hasThreeDigitFactors(int);

int main()
{
    static int largestNum, index;
    largestNum = 999; //largest 3-digit number
    index = largestNum * largestNum; //the largest product of two 3-digit numbers
    while (index > 10000) {
        if (isPalindrome(index)){ //check to see if current num is palindrome
            if (hasThreeDigitFactors(index)){
                printf("%d\n", index);
                break;
            }
        }
        index--;
    }
    return 0;
}

bool hasThreeDigitFactors(int target){
    int min = 100;
    int max = 999;
    int i;
    int quotient;
    for(i = max; i >= min; i--) {
        if (target % i == 0) {
            quotient = target / i;
            if (quotient > 100 && quotient <= 999) {
                    return true;
            }
        }
        
    }
    return false;

}

bool isPalindrome(int target){
    char num[6];
    int front = 0, end = strlen(num) - 1;

    sprintf(num, "%d", target); //print the current num to a string, str
    while(front < (signed)strlen(num)/2){
        if (num[front] != num[end]){ //if front does not equal back --> not palindrome
            return false;
        }
        front++;
        end--;
    }
    return true; //if it makes it this far, its a palindrome
}
