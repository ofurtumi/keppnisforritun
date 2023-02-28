#include <iostream>

using namespace std;

int main()
{
    char arr[5][6];
    for (int i = 0; i < 5; i++)
    {
        cin >> arr[i];
    }

    int count = 0;
    bool invalid = false;
    for (int i = 0; i < 5; i++)
    {
        for (int j = 0; j < 5; j++)
        {
            if (arr[i][j] == 'k')
            {
                count += 1;
                int pos11 = i + 2, pos12 = j + 1;
                if (pos11 < 5 && pos12 < 5 && arr[pos11][pos12] == 'k')
                    invalid = true;

                int pos21 = i + 1, pos22 = j + 2;
                if (pos21 < 5 && pos22 < 5 && arr[pos21][pos22] == 'k')
                    invalid = true;

                int pos31 = i - 2, pos32 = j + 1;
                if (pos31 >= 0 && pos32 < 5 && arr[pos31][pos32] == 'k')
                    invalid = true;

                int pos41 = i - 1, pos42 = j + 2;
                if (pos41 >= 0 && pos42 < 5 && arr[pos41][pos42] == 'k')
                    invalid = true;
            }
        }
    }
    if (invalid || count < 9)
        cout << "invalid" << endl;
    else
        cout << "valid" << endl;
    return 0;
}