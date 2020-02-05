// C Solution For Largest Palindrome Product of Two 3-Digit Numbers

// Avg. run-time of about 500ms

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

bool isPalindrome(char []);

int main()
{
    static int largestNum, index, result = 0;
    char str[6];
    largestNum = 999; //largest 3-digit number
    index = largestNum * largestNum; //the largest product of two 3-digit numbers
    while (index > 10000) {
        sprintf(str, "%d", index); //print the current num to a string, str
        if (isPalindrome(str)){ //check to see if current num is palindrome
            for(int i = largestNum; i >= 100; i--){ //see if any 3-digit nums are factors of current num
                if (index % i == 0 && (index / i) < (largestNum + 1) && (index / i) > 99){ //check to see if both factors are 3-digit
                    result = index;
                    printf("%d\n", result);
                    break;
                }
            }
        }
        if (result > 0) break; //we can end the loop bc we have the ans
        index--;
    }
    return 0;
}

bool isPalindrome(char num[]){
    int front = 0, end = strlen(num) - 1;
    while(front < (signed)strlen(num)/2){
        if (num[front] != num[end]){ //if front does not equal back --> not palindrome
            return false;
        }
        front++;
        end--;
    }
    return true; //if it makes it this far, its a palindrome
}
