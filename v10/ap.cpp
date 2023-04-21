#include <cstdlib>
#include <iostream>
#include <stdio.h>
#include <cmath>
using namespace std;

int main() {
    int num;
    while(cin >> num) {
        int sum = 1;
        int upperBound = sqrt(num);
        cout << upperBound << endl;
        for(int lowerBound = 2; lowerBound <= upperBound; lowerBound++) {
            if (num % lowerBound == 0) {
                if(lowerBound != num/lowerBound) {
                    sum = sum + lowerBound + num/lowerBound;   
                }
                else sum = sum + lowerBound;             
            }
        }
        cout << sum << endl;
        if (sum == num) cout << num << " perfect" << endl;
        else if (num - sum <= 2 && num - sum >= -2) cout << num << " almost perfect" << endl;
        else cout << num << " not perfect" << endl;
    }
    return 0;
}
