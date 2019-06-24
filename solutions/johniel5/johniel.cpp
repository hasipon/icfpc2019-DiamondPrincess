#include "solve.hpp"

const int H = 500;
const int W = 500;

struct Worker {
  int i, j;
  int dir;
  vector<pair<int,int>> manip;
  vector<pair<int, int>> pend;
  map<char, int> boosters;

  Worker()
  {
    dir = 0;
    static int di[4][4] = {{0, -1,  0, +1}, {0, +1, +1, +1}, {0, -1,  0, +1}, {0, -1, -1, -1}};
    static int dj[4][4] = {{0, +1, +1, +1}, {0, -1,  0, +1}, {0, -1, -1, -1}, {0, -1,  0, +1}};

    pend.push_back({0, -1});
    pend.push_back({-1, 0});
    pend.push_back({+1, 0});

    // pend.push_back({-2, +1});
    // pend.push_back({+2, +1});

    for (int k = 0; k < 4; ++k) {
      manip.push_back({di[0][k], dj[0][k]});
    }
  }
};

Worker addManip(Worker w)
{
  assert(0 < w.boosters['B']);
  --w.boosters['B'];
  pair<int, int> p = w.pend.back();
  w.pend.pop_back();
  for (int i = 0; i < w.dir; ++i) {
    int ni =  p.second;
    int nj = -p.first;
    p = make_pair(ni, nj);
  }
  w.manip.push_back(p);
  return w;
}

Worker clockwise(Worker w)
{
  w.dir = (w.dir + 1) % 4;
  each (i, w.manip) {
    int ni =  i.second;
    int nj = -i.first;
    i = make_pair(ni, nj);
  }
  return w;
}

Worker counterclockwise(Worker w)
{
  w = clockwise(w);
  w = clockwise(w);
  w = clockwise(w);
  return w;
}

Worker move(char c, Worker w)
{
  switch (c) {
    case 'W': --w.i; break;
    case 'S': ++w.i; break;
    case 'A': --w.j; break;
    case 'D': ++w.j; break;
    case 'E':
      w = clockwise(w);
      break;
    case 'Q':
      w = counterclockwise(w);
      break;
  }
  return w;
}

static bool colored[H][W];
string bfs(pair<int, int> src, const M& m)
{
  set<pair<int, int>> vis;
  vis.insert(src);

  const int di[] = {0, 0, -1, +1};
  const int dj[] = {-1, +1, 0, 0};

  static pair<int, int> path[H][W];
  map<pair<int, int>, char> cmd;
  string C = "ADWS";

  const pair<int, int> def = {-1, -1};
  pair<int, int> dst = def;
  queue<pair<int, int>> q;
  for (q.push(src); q.size(); q.pop()) {
    auto curr = q.front();
    // clog << curr.first << ' ' << curr.second << endl;
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
      if (!colored[ni][nj]) {
        dst = {ni, nj};
        break;
      }
    }
    if (dst != def) break;
  }
  //clog << "dst: " << dst.first << ' ' << dst.second << endl;
  if (dst == def) return "";
  string s;
  while (src != dst) {
    s += cmd[dst];
    dst = path[dst.first][dst.second];
  }
  reverse(s.begin(), s.end());
  // clog << "path: " << s << endl;
  return s;
}

void solve(const M& m)
{
  vector<pair<int, int>> v;
  for (int i = 0; i < m.h(); ++i) {
    for (int j = 0; j < m.w(); ++j) {
      unless (m.blocked(i, j)) v.push_back({i, j});
    }
  }
  fill(&colored[0][0], &colored[H - 1][W - 1] + 1, false);

  Worker curr;
  curr.i = m.ini.first;
  curr.j = m.ini.second;
  curr.dir = 0;

  const string C = "EQDSAW";

  auto color_cnt = [&] (Worker curr) {
    if (!m.inside(curr.i, curr.j)) return -(1 << 28);
    if (m.blocked(curr.i, curr.j)) return -(1 << 28);
    int cnt = 0;
    each (i, curr.manip) {
      int ni = curr.i + i.first;
      int nj = curr.j + i.second;
      cnt += m.inside(ni, nj) && !m.blocked(ni, nj) && !colored[ni][nj];
    }
    return cnt;
  };

  auto color = [&] (void) {
    each (i, curr.manip) {
      int ni = curr.i + i.first;
      int nj = curr.j + i.second;
      colored[ni][nj] = (m.inside(ni, nj) && !m.blocked(ni, nj));
    }
    return ;
  };

  map<pair<int, int>, vector<char>> bs;
  each (b, m.boosters) bs[{b.y, b.x}].push_back(b.c);

  char buff[100];

  color();
  string t;
  while (true) {
    if (0 < curr.boosters['B'] && curr.pend.size()) {
      auto tmp = addManip(curr);
      auto p = tmp.manip.back();
      p.first += tmp.i;
      p.second += tmp.j;
      if (m.inside(p.first, p.second) && !m.blocked(p.first, p.second)) {
        curr = tmp;
        clog << curr.manip.back().second << ' ' <<  -curr.manip.back().first << endl;
        sprintf(buff, "B(%d,%d)", curr.manip.back().second, -curr.manip.back().first);
        t += string(buff);
        color();
        continue;
      }
    }

    pair<int, char> mx = {-1, '@'};
    each (c, C) {
      Worker next = move(c, curr);
      mx = max(mx, make_pair(color_cnt(next), c));
    }

    if (mx.first <= 0) {
      while (v.size() && colored[v.back().first][v.back().second]) v.pop_back();
      if (v.empty()) break;
      pair<int, int> dst = v.back();
      string s = bfs(make_pair(curr.i, curr.j), m);
      each (c, s) {
        if (colored[dst.first][dst.second]) break;
        curr = move(c, curr);
        color();
        t += c;
        each (i, bs[make_pair(curr.i, curr.j)]) ++curr.boosters[i];
        bs[make_pair(curr.i, curr.j)].clear();
      }
    } else {
      curr = move(mx.second, curr);
      color();
      t += mx.second;
      each (i, bs[make_pair(curr.i, curr.j)]) ++curr.boosters[i];
      bs[make_pair(curr.i, curr.j)].clear();

    }
  }
  cout << t << endl;
  clog << t.size() << endl;

  return ;
}
