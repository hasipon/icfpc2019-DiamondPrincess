#include "solve.hpp"

const int H = 500;
const int W = 500;

using P = struct {
  int x, y;
};
bool operator < (P a, P b)
{
  return make_pair(a.x, a.y) < make_pair(b.x, b.y);
}

string bfs(pair<int, int> src, pair<int, int> dst, const M& m)
{
  set<pair<int, int>> vis;
  vis.insert(src);

  const int di[] = {0, 0, -1, +1};
  const int dj[] = {-1, +1, 0, 0};

  static pair<int, int> path[H][W];
  map<pair<int, int>, char> cmd;
  string C = "ADWS";

  queue<pair<int, int>> q;
  for (q.push(src); q.size(); q.pop()) {
    if (vis.count(dst)) break;
    auto curr = q.front();
    for (int d = 0; d < 4; ++d) {
      int ni = curr.first + di[d];
      int nj = curr.second + dj[d];
      unless (m.inside(ni, nj)) continue;
      if (m.blocked(ni, nj)) continue;
      auto next = make_pair(ni, nj);
      if (vis.count(next)) continue;
      q.push(next);
      vis.insert(next);
      path[ni][nj] = curr;
      cmd[next] = C[d];
      // cout << curr.first << ' ' << curr.second << " -> " << next.first << ' ' << next.second << endl;
    }
  }

  string s;
  while (src != dst) {
    s += cmd[dst];
    dst = path[dst.first][dst.second];
  }
  reverse(s.begin(), s.end());
  // cout << s << endl;
  return s;
}

void solve(const M& m)
{
  vector<pair<int, int>> v;
  for (int i = 0; i < m.h(); ++i) {
    for (int j = 0; j < m.w(); ++j) {
      if (!m.blocked(i, j)) {
        v.push_back({i, j});
      }
    }
  }
  sort(v.begin(), v.end());

  static bool colored[H][W];
  fill(&colored[0][0], &colored[H - 1][W - 1] + 1, false);

  pair<int, int> curr = m.ini;
  int dir = 0;

  auto color = [&] (void) {
    int di[] = {0, -1, 0, +1};
    int dj[] = {0, +1, +1, +1};
    for (int d = 0; d < 4; ++d) {
      int ni = curr.first + di[d];
      int nj = curr.second + dj[d];
      if (m.inside(ni, nj)) {
        colored[ni][nj] = true;
      }
    }
    return ;
  };

  auto move = [&] (char c) {
    	switch (c) {
        case 'W': --curr.first; break;
        case 'S': ++curr.first; break;
        case 'A': --curr.second; break;
        case 'D': ++curr.second; break;
        case 'E': dir = (dir + 1) % 4; break;
        case 'Q': dir = (dir + 3) % 4; break;
      }
      return ;
  };

  string t;
  color();
  while (v.size()) {
    sort(v.begin(), v.end(), [&] (auto a, auto b) {
      int c = abs(a.first - curr.first) + abs(a.second - curr.second);
      int d = abs(b.first - curr.first) + abs(b.second - curr.second);
      if (c == d) return a < b;
      return c > d;
    });
    pair<int, int> dst = v.back();
    string s = bfs(curr, dst, m);
    each (c, s) {
      if (colored[dst.first][dst.second]) break;
      move(c);
      color();
      t += c;
    }
    while (v.size() && colored[v.back().first][v.back().second]) v.pop_back();
  }
  cout << t << endl;

  return ;
}
