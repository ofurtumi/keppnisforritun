// Tiledes
#include <iostream>
#include <vector>

// https://github.com/stevenhalim/cpbook-code/blob/master/ch2/ourown/unionfind_ds.cpp (modified)
typedef std::vector<int> vi;

class UnionFind {  // OOP style
private:
    vi p, rank, setSize;  // vi p is the key part
    int numSets;

public:
    UnionFind(int N) {
        p.assign(N + 1, 0);
        for (int i = 1; i < N + 1; ++i) p[i] = i;
        rank.assign(N + 1, 0);     // optional speedup
        setSize.assign(N + 1, 1);  // optional feature
        numSets = N;               // optional feature
    }

    int findSet(int i) { return (p[i] == i) ? i : (p[i] = findSet(p[i])); }
    bool isSameSet(int i, int j) { return findSet(i) == findSet(j); }

    int numDisjointSets() { return numSets; }             // optional
    int sizeOfSet(int i) { return setSize[findSet(i)]; }  // optional

    void unionSet(int i, int j) {
        if (isSameSet(i, j))
            return;                          // i and j are in same set
        int x = findSet(i), y = findSet(j);  // find both rep items
        if (rank[x] > rank[y])
            std::swap(x, y);  // keep x 'shorter' than y
        p[x] = y;             // set x under y
        if (rank[x] == rank[y])
            ++rank[y];             // optional speedup
        setSize[y] += setSize[x];  // combine set sizes at y
        --numSets;                 // a union reduces numSets
    }
};

int main(void) {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(NULL);

    int n, q;
    std::cin >> n >> q;
    UnionFind guests(n);
    for (int i = 0; i < q; ++i) {
        char instruction;
        std::cin >> instruction;
        switch (instruction) {
        case 's':
            int target;
            std::cin >> target;
            std::cout << guests.sizeOfSet(target) << '\n';
            break;
        case 't':
            int a, b;
            std::cin >> a >> b;
            guests.unionSet(a, b);
            break;
        }
    }
    return 0;
}