#include <stdio.h>
#include <stdlib.h>

int main() {
    char *input_name = (char*)malloc(sizeof(char*)*64);
    char stored_name[1];
    char my_name[32];

    printf("What is your first name? ");
    scanf("%s", input_name);
    strcpy(stored_name, input_name);
    printf("What is your first name? %s\n", stored_name);
    printf("My name: %s\n", my_name);

    if (strcmp(my_name, "bob") == 0) {
        printf("You've successfully passed! Here's the flag: ");
    } else {
        printf("Sorry, %s, looks like you aren't special enough to get the flag.", stored_name);
    }
}