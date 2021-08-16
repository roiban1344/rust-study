#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <vector>
using namespace std;

int transform(int x) {
    vector<int> a;
    while(x > 0) {
        int q = x / 10;
        a.push_back(x - q * 10);
        x = q;
    }
    sort(a.begin(), a.end());
    int y = 0;
    for(auto it = a.rbegin(); it < a.rend(); ++it) {
        y = y * 10 + *it;
    }
    int z = 0;
    for(auto it = a.begin(); it < a.end(); ++it) {
        z = z * 10 + *it;
    }
    return y - z;
}

int main() {
    map<int, set<int>> fixed_from;
    for(int n = 1'000; n < 10'000; ++n) {
        int m = n;
        vector<int> met;
        while(find(met.begin(), met.end(), m) == met.end()) {
            met.push_back(m);
            m = transform(m);
        }
        if(fixed_from.count(m)) {
            fixed_from.at(m).insert(n);
        } else {
            fixed_from.insert(make_pair(m, set<int>({n})));
        }
    }
    for(auto x : fixed_from) {
        printf("%d %d\n", x.first, x.second.size());
    }
    return 0;
}
