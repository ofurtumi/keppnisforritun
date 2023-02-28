#include <iostream>
#include <string.h>

int main()
{
    int n;
    std::cin >> n;

    for (int i = 1; i <= n; i++)
    {
        char input[7];
        std::cin >> input;

        if (strcmp(input, "mumble") && atoi(input) != i)
        {
            std::cout << "something is fishy" << std::endl;
            return 0;
        }
    }

    std::cout << "makes sense" << std::endl;
    return 0;
}
