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

// ll ind(ll x, ll y) { return x * (1000000001) + y; }
ll ind(P p) { return p.first * (1000000111) + p.second; }

vector<P> search(P p, unordered_map<ll, set<ll>> &x2y,
                 unordered_map<ll, set<ll>> &y2x) {
    vector<P> res;
    // x ue
    ll x = p.first;
    ll y = p.second;
    auto iter4 = y2x[y].lower_bound(x);
    if (iter4 != y2x[y].begin()) {
        ll val4 = *(--iter4);
        res.push_back({val4 + 1, y});
    }

    // x sita
    auto iter5 = y2x[y].upper_bound(x);
    if (iter5 != y2x[y].end()) {
        ll val4 = *(iter5);
        res.push_back({val4 - 1, y});
    }

    // y left

    auto iter6 = x2y[x].lower_bound(y);
    if (iter6 != x2y[x].begin()) {
        ll val4 = *(--iter6);
        res.push_back({x, val4 + 1});
    }

    // y right
    auto iter7 = x2y[x].upper_bound(y);
    if (iter7 != x2y[x].end()) {
        ll val4 = *(iter7);
        res.push_back({x, val4 - 1});
    }

    return res;
}

void solve() {
    ll h, w, n;
    cin >> h >> w >> n;
    ll sx, sy;
    cin >> sx >> sy;
    ll gx, gy;
    cin >> gx >> gy;
    vector<P> xyl;
    unordered_map<ll, set<ll>> x2y;
    unordered_map<ll, set<ll>> y2x;
    REP(i, n) {
        ll x, y;
        cin >> x >> y;
        xyl.push_back({x, y});
        x2y[x].insert(y);
        y2x[y].insert(x);
    }
    unordered_map<ll, vector<P>> gl;
    unordered_map<ll, ll> dist;
    dist[ind({sx, sy})] = 0;
    dist[ind({gx, gy})] = INF;

    vector<P> to = search({sx, sy}, x2y, y2x);
    for (auto &t : to) {
        gl[ind({sx, sy})].push_back(t);
        dist[ind(t)] = INF;
    }

    vector<P> dd = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
    for (auto &[x, y] : xyl) {
        for (auto &[dx, dy] : dd) {
            ll xx = x + dx;
            ll yy = y + dy;
            // if (xx <= 0 || xx > h || yy <= 0 || yy > w) {
            //     continue;
            // }
            dist[ind({xx, yy})] = INF;
            vector<P> to = search({xx, yy}, x2y, y2x);
            for (auto &t : to) {
                gl[ind({xx, yy})].push_back(t);
                dist[ind(t)] = INF;
            }
        }
    }

    dist[ind({sx, sy})] = 0;

    deque<P> q;
    q.push_back({sx, sy});
    while (!q.empty()) {
        P pos = q.front();
        // DEBUG(pos);
        // DEBUG(gl[ind(pos)]);
        q.pop_front();
        // vector<P> neibs = gl[pos]
        for (P &neib : gl[ind(pos)]) {
            if (dist[ind(neib)] != INF) {
                continue;
            }
            dist[ind(neib)] = dist[ind(pos)] + 1;
            q.push_back(neib);
        }
    }

    if (dist[ind({gx, gy})] == INF) {
        cout << -1 << endl;
    } else {
        cout << dist[ind({gx, gy})] << endl;
    }

    // for (auto &p : gl) {
    //     ll po = {p.first, p.second};
    //     dist[p] = INF;
    // }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    DEBUG("local");
    solve();
    return 0;
}