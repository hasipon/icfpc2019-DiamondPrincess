#include "solve.hpp"

const int H = 500;
const int W = 500;

struct Worker {
  int i, j;
  int dir;
};

pair<int,int> rotate(pair<int,int> v, int dir) {
	switch (dir) {
	case 0: return v;
	case 1: return {v.second, -v.first};
	case 2: return {-v.first, -v.second};
	case 3: return {-v.second, v.first};
	default: throw 1;
	}
}

static bool colored[H][W];
string bfs_near(pair<int, int> src, pair<int, int> dst, const M& m)
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
      if(make_pair(ni, nj) == dst) goto BFS_END;
    }
  }
  BFS_END:
  string s;
  while (src != dst) {
    s += cmd[dst];
    dst = path[dst.first][dst.second];
  }
  reverse(s.begin(), s.end());
  return s;
}

pair<int, int> bfs_far(pair<int, int> src, const M& m)
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
      }
    }
  }
  return dst;
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

  const string C = "DSAWEQ";
  int di[4][4] = {{0, -1,  0, +1}, {0, +1, +1, +1}, {0, -1,  0, +1}, {0, -1, -1, -1}};
  int dj[4][4] = {{0, +1, +1, +1}, {0, -1,  0, +1}, {0, -1, -1, -1}, {0, -1,  0, +1}};

  auto color_cnt = [&] (Worker curr) {
    if (!m.inside(curr.i, curr.j)) return -(1 << 28);
    if (m.blocked(curr.i, curr.j)) return -(1 << 28);
    int cnt = 0;
    for (int d = 0; d < 4; ++d) {
      int ni = curr.i + di[curr.dir][d];
      int nj = curr.j + dj[curr.dir][d];
      cnt += m.inside(ni, nj) && !m.blocked(ni, nj) && !colored[ni][nj];
    }
    return cnt;
  };

  auto color = [&] (void) {
    for (int d = 0; d < 4; ++d) {
      int ni = curr.i + di[curr.dir][d];
      int nj = curr.j + dj[curr.dir][d];
      colored[ni][nj] = (m.inside(ni, nj) && !m.blocked(ni, nj));
    }
    return ;
  };

  auto move = [&] (char c, Worker curr) {
    	switch (c) {
        case 'W': --curr.i; break;
        case 'S': ++curr.i; break;
        case 'A': --curr.j; break;
        case 'D': ++curr.j; break;
        case 'E': curr.dir = (curr.dir + 1) % 4; break;
        case 'Q': curr.dir = (curr.dir + 3) % 4; break;
      }
      return curr;
  };

  color();
  string t;
  while (true) {
    pair<int, char> mx = {-1, '@'};
    each (c, C) {
      Worker next = move(c, curr);
      mx = max(mx, make_pair(color_cnt(next), c));
    }
    if(mx.first > 0){
      curr = move(mx.second, curr);
      color();
      t += mx.second;
    }else {
      while (v.size() && colored[v.back().first][v.back().second])
        v.pop_back();
      if (v.empty())
        break;
      auto far_dst = bfs_far(make_pair(curr.i, curr.j), m);
      auto far_far_dst = bfs_far(far_dst, m);
      string s;
      if (far_far_dst.first == -1) {
        s = bfs_near(make_pair(curr.i, curr.j), far_dst, m);
      } else if (far_far_dst == make_pair(curr.i, curr.j)) {
        s = bfs_near(make_pair(curr.i, curr.j), far_dst, m);
      } else {
        s = bfs_near(make_pair(curr.i, curr.j), far_far_dst, m);
      }
      if (s.empty())
        break;
      int cnt = 0;
      each(c, s) {
        cnt++;
        curr = move(c, curr);
        color();
        t += c;
        if (cnt >= 5)
          break;
      }
    }
  }
  cout << t << endl;
  // clog << t << endl;
  // clog << t.size() << endl;

  return ;
}
