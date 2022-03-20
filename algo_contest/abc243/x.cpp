#include <bits/stdc++.h>
#if __has_include(<atcoder/all>)
#include <atcoder/all>
using namespace atcoder;
#endif
#if __has_include(<local_debug.hpp>)
#include <local_debug.hpp>
#else
#define DEBUG(...)
#endif

using namespace std;
typedef long long ll;
typedef pair<ll, ll> P;
#define FOR(i, a, b) for (ll i = a; i < b; i++)   // for i in range(a,b)
#define REP(i, n) for (ll i = 0; i < n; i++)      // for i in range(b)
#define FORD(i, a, b) for (ll i = a; i > b; i--)  // for i in range(a,b,-1)
#define ALL(x) x.begin(), x.end()

const int IINF = 1'001'001'001;
const ll INF = 1'001'001'001'001'001'001ll;
const ll MOD = 1'000'000'007;
// const ll MOD = 998244353;
// using mint = modint1000000007;
// using mint = modint998244353;

constexpr int logn = 20;
constexpr int maxn = 1 << logn;

ll hilbertorder(int x, int y) {
    ll d = 0;
    for (int s = 1 << logn - 1; s; s >>= 1) {
        bool rx = x & s, ry = y & s;
        DEBUG(d);
        d = d << 2 | rx * 3 ^ static_cast<int>(ry);
        DEBUG(rx, ry, d);
        if (ry) continue;
        if (rx) {
            x = maxn - x;
            y = maxn - y;
        }
        swap(x, y);
    }
    return d;
}

ll triangleorder(int l, int r) {
    ll d = 0;
    for (ll s = 1 << logn - 1; s > 1; s >>= 1) {
        if (l >= s) {
            d += (36 * s * s) >> 4;
            l -= s;
            r -= s;
        } else if (l + r > s << 1) {
            d += (24 * s * s) >> 4;
            l = l + 1;
            r = (s << 1) - r;
            swap(l, r);
        } else if (r > s) {
            d += (12 * s * s) >> 4;
            l = s - l;
            r = r - s - 1;
            swap(l, r);
        }
    }
    d += l + r - 1;
    return d;
}

void solve() {
    // DEBUG(0 * 3 ^ 1);
    DEBUG(triangleorder(5, 10));
    DEBUG(triangleorder(6, 11));
    DEBUG(triangleorder(7, 12));

    //
    // hilbertorder(5, 10);
    // ll x = 1234567891234567891;
    // ll xsq = sqrt(x);
    // ll xsq1 = sqrt(x) + 1;
    // DEBUG(xsq * xsq);
    // DEBUG(xsq1 * xsq1);

    // DEBUG(1 << 3 - 1);
    // DEBUG(1 << 1 | 3 * 4);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    DEBUG("local");
    solve();
    return 0;
}