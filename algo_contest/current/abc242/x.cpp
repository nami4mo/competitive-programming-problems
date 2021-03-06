#include <bits/stdc++.h>
using namespace std;
typedef signed long long ll;

#undef _P
#define _P(...) (void)printf(__VA_ARGS__)
#define FOR(x, to) for (x = 0; x < (to); x++)
#define FORR(x, arr) for (auto& x : arr)
#define ITR(x, c) for (__typeof(c.begin()) x = c.begin(); x != c.end(); x++)
#define ALL(a) (a.begin()), (a.end())
#define ZERO(a) memset(a, 0, sizeof(a))
#define MINUS(a) memset(a, 0xff, sizeof(a))
//-------------------------------------------------------

int N, Q;
int C[1000011];
int L[1000011], R[1000011];
const int DI = 800;
vector<pair<int, int>> ev[DI];
int ret[1000011];
int num[1000011];

void solve() {
    int i, j, k, l, r, x, y;
    string s;

    // cin >> N >> Q;
    // FOR(i, N) cin >> C[i];
    // FOR(i, Q) {
    //     cin >> L[i] >> R[i];
    //     L[i]--;
    //     ev[L[i] / DI].push_back({R[i], i});
    // }
    cin >> N;
    FOR(i, N) cin >> C[i];
    cin >> Q;
    FOR(i, Q) {
        cin >> L[i] >> R[i];
        L[i]--;
        ev[L[i] / DI].push_back({R[i], i});
    }

    FOR(i, DI) {
        ZERO(num);
        int c = 0;
        sort(ALL(ev[i]));
        int CL = 0, CR = 0;
        FORR(e, ev[i]) {
            x = e.second;
            while (CR < R[x]) {
                num[C[CR]]++;
                if (num[C[CR]] % 2 == 0) c++;
                CR++;
            }
            while (CL < L[x]) {
                num[C[CL]]--;
                if (num[C[CL]] % 2 == 1) c--;
                CL++;
            }
            while (L[x] < CL) {
                CL--;
                num[C[CL]]++;
                if (num[C[CL]] % 2 == 0) c++;
            }
            ret[x] = c;
        }
    }
    FOR(i, Q) cout << ret[i] << endl;
}

int main(int argc, char** argv) {
    string s;
    int i;
    if (argc == 1) ios::sync_with_stdio(false), cin.tie(0);
    FOR(i, argc - 1) s += argv[i + 1], s += '\n';
    FOR(i, s.size()) ungetc(s[s.size() - 1 - i], stdin);
    cout.tie(0);
    solve();
    return 0;
}
