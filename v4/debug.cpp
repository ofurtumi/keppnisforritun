#include <cstdio>
#include <cstring>
#include <iostream>
#include <algorithm>
#include <vector>
#include <map>
#include <queue>
using namespace std;
typedef long long ll;
const int N = 1000005;
const ll inf = 0x3f3f3f3f3f3f3f3f;
ll dp[N];
ll n, p, r;

ll dfs(ll n)
{
    if (n <= 1)
        return 0;
    if (dp[n] != inf)
        return dp[n];
    for (ll i = 1; i < n; i++)
    {
        dp[n] = min(dp[n], i * p + dfs((n + i / (i + 1)) + r));
    }
    return dp[n];
}

int main()
{
    cin >> n >> r >> p;
    for (int i = 0; i <= n; i++)
        dp[i] = inf;
    cout << dfs(n);
}