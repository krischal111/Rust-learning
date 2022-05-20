#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <stdlib.h>

const char * names[] = {
    "Sa",
    "(Komal)Re",
    "Re",
    "(Komal)Ga",
    "Ga",
    "Ma",
    "Ma(Tivra)",
    "Pa",
    "(Komal)Dha",
    "Dha",
    "(Komal)Ni",
    "Ni",
    "Sa"
};

int key_to_shift(int a)
{
    int shiftvalue;
    switch(toupper(a))
    {
        case (int)'E':
        shiftvalue = 4;
        break;

        case 'B':
        shiftvalue = -1;
        break;

        case 'G':
        shiftvalue = -5;
        break;

        case 'D':
        shiftvalue = 2;
        break;

        case 'A':
        shiftvalue = -3;    
        break;
    }
    
    return shiftvalue;
}

int main()
{
    char tabs[200];
    int key, shiftvalue;
    printf("Please enter key and frets of guitar: ");
    fgets(tabs,99,stdin);
    key = tabs[0];

    char *str = tabs;
    char* token;
    char* rest = str;
 
    while ((token = strtok_r(rest, " ", &rest)))
    {
        if(isalpha(*token))
        {
            shiftvalue = key_to_shift((int)(*token));
            printf("\n");
            continue;
        }
        int a = atoi(token);
        int tone = ((a + shiftvalue)%12+12)%12;
        printf("%s, ",names[tone]);
        // printf("%s\n", token);
    }
    return (0);

}