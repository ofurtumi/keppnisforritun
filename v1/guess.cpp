#include <iostream>

int main() {
    int high = 1001, low = 1, guess = 500;
    std::cout << guess << std::endl;

    for (int i = 0; i < 10; i++)
    {
        std::cout.flush();
        char command[10];
        std::cin >> command;
        if (command[0] == 'l') high = guess;
        else if (command[0] == 'h') low = guess;
        else return 0;

        guess = low + (high - low) / 2;

        std::cout << guess << std::endl;
    }
    return 0; 
}