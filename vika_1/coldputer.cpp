#include <iostream>

int main() {
    int n, count = 0;
    std::cin >> n;

    for (int i = 0; i < n; i++) {
        int temp;
        std::cin >> temp;
        if (temp < 0) count++;
    }

    std::cout << count << std::endl;
    return 0;
}