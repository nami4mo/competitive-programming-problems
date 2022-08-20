#include <bits/stdc++.h>
using namespace std;
using ll = long long;
ll modPow(ll a, ll n, ll mod) {
    if (mod == 1) return 0;
    ll ret = 1;
    ll p = a % mod;
    while (n) {
        if (n & 1) ret = ret * p % mod;
        p = p * p % mod;
        n >>= 1;
    }
    return ret;
}

ll dp[3101][3101];
ll sum[3101][3101];
int main() {
    ll n, p;
    cin >> n >> p;
    dp[0][0] = modPow(25, p - 2, p) * 26ll % p;
    vector<ll> ten = {1, 10, 100, 1000, 10000, 100000};
    for (int i = 1; i <= n; i++) sum[0][i] = dp[0][0];
    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= n; j++) {
            for (int k = 1; k <= 4; k++) {
                if (i - k - 1 < 0) continue;
                ll x = max(j - ten[k - 1] + 1, 0ll),
                   y = max(j - ten[k] + 1, 0ll);
                dp[i][j] += (sum[i - k - 1][x] - sum[i - k - 1][y] + p) * 25;
                dp[i][j] %= p;
            }
            sum[i][j + 1] = sum[i][j] + dp[i][j];
            sum[i][j + 1] %= p;
        }
    }
    ll sum = 0;
    for (int i = 1; i < n; i++) sum += dp[i][n];
    cout << sum % p << endl;
}
