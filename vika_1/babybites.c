#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    int n;
    int ok = 1;
    scanf("%d", &n);

    for (int i = 0; i < n; i++)
    {
        char temp[7];
        scanf("%s", temp);

        if (strcmp(temp, "mumble") != 0 && atoi(temp) != i + 1)
        {
            printf("something is fishy\n");
            return 0;
        }
    }

    printf("makes sense\n");
    return 0;
}