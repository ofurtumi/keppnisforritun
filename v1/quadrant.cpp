#include <iostream>

int main() {
    int x, y, q;
    std::cin >> x >> y;

    if (x > 0 && y > 0) q = 1;
    else if (x < 0 && y > 0) q = 2;
    else if (x < 0 && y < 0) q = 3;
    else q = 4;

    std::cout << q << std::endl;
    return 0;
}