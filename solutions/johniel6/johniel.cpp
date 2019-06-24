#include "solve.hpp"

vector<pair<int,int>> calc_obstacles(pair<int,int> v) {
	int dy = v.first;
	int dx = v.second;
	bool flip_xy = false;
	bool flip_y = false;
	bool flip_x = false;
	if (abs(dy) < abs(dx)) {
		swap(dy, dx);
		flip_xy = true;
	}
	if (dy < 0) {
		dy = -dy;
		flip_y = true;
	}
	if (dx < 0) {
		dx = -dx;
		flip_x = true;
	}
	if (dy <= 1) return {};
	vector<pair<int,int>> result;
	for (int y = 1; y < dy; ++ y) {
		int x1 = ((2*y-1)*dx+dy) / (2*dy);
		int x2 = ((2*y+1)*dx+dy-1) / (2*dy);
		for (int x = x1; x <= x2; ++ x) {
			int yy = y;
			int xx = x;
			if (flip_x) xx = -xx;
			if (flip_y) yy = -yy;
			if (flip_xy) swap(xx,yy);
			result.push_back({yy,xx});
		}
	}
	return result;
}

const int H = 500;
const int W = 500;

pair<int, int> rotR(pair<int, int> p)
{
  return {p.second, -p.first};
}

struct Manip {
  int i, j; // head
  vector<pair<int, int>> req; // arm

  Manip(int i_, int j_)
  {
    i = i_;
    j = j_;

    req = calc_obstacles({j, i});

    each (i, req) swap(i.first, i.second);
  }

  bool reachable(const M& m, int base_i, int base_j) const
  {
    if (!m.inside(i + base_i, j + base_j)) return false;
    if (m.blocked(i + base_i, j + base_j)) return false;
    each (k, req) if (m.blocked(k.first + base_i, k.second + base_j)) return false;
    return true;
  }
};

Manip clockwise(Manip m)
{
  int ni =  m.j;
  int nj = -m.i;

  m.i = ni;
  m.j = nj;

  each (i, m.req) i = rotR(i);
  return m;
}

Manip counterclockwise(Manip m)
{
  m = clockwise(m);
  m = clockwise(m);
  m = clockwise(m);
  return m;
}

struct Worker {
  int i, j, dir;
  vector<Manip> manips;
  vector<Manip> pend;
  map<char, int> boosters;

  Worker(int _i, int _j, int _dir)
  {
    i = _i;
    j = _j;
    dir = _dir;
    static int di[4][4] = {{0, -1,  0, +1}, {0, +1, +1, +1}, {0, -1,  0, +1}, {0, -1, -1, -1}};
    static int dj[4][4] = {{0, +1, +1, +1}, {0, -1,  0, +1}, {0, -1, -1, -1}, {0, -1,  0, +1}};

    for (int k = 0; k < 4; ++k) {
      manips.push_back(Manip(di[0][k], dj[0][k]));
    }

    pend.push_back(Manip(-4, +1));
    pend.push_back(Manip(+4, +1));
    pend.push_back(Manip(-3, +1));
    pend.push_back(Manip(+3, +1));
    pend.push_back(Manip(-2, +1));
    pend.push_back(Manip(+2, +1));
  }
};

Worker addManip(Worker w)
{
  assert(0 < w.boosters['B']);
  --w.boosters['B'];
  Manip m = w.pend.back();
  w.pend.pop_back();
  for (int i = 0; i < w.dir; ++i) {
    m = clockwise(m);
  }
  w.manips.push_back(m);
  return w;
}

Worker clockwise(Worker w)
{
  w.dir = (w.dir + 1) % 4;
  each (i, w.manips) i = clockwise(i);
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
int wrappable(const M& m, const Worker& w)
{
  unless (m.inside(w.i, w.j)) return -(1 << 28);
  if (m.blocked(w.i, w.j)) return -(1 << 28);

  int cnt = 0;
  each (k, w.manips) {
    if (k.reachable(m, w.i, w.j)) {
      // clog << k.i + w.i << ' ' << k.j + w.j << ", " << colored[k.i + w.i][k.j + w.j] << endl;
      cnt += (colored[k.i + w.i][k.j + w.j] == false);
    }
  }
  return cnt;
}
int wrap(const M& m, const Worker& w)
{
  int cnt = 0;
  each (k, w.manips) {
    if (k.reachable(m, w.i, w.j)) {
      cnt += (colored[k.i + w.i][k.j + w.j] = true);
    }
  }
  return cnt;
}

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
  if (dst == def) return "";
  string s;
  while (src != dst) {
    s += cmd[dst];
    dst = path[dst.first][dst.second];
  }
  reverse(s.begin(), s.end());
  return s;
}

void solve(const M& m)
{
  const string C = "EQDSAW";
  char buff[100];

  vector<pair<int, int>> v;
  for (int i = 0; i < m.h(); ++i) {
    for (int j = 0; j < m.w(); ++j) {
      unless (m.blocked(i, j)) v.push_back({i, j});
    }
  }
  fill(&colored[0][0], &colored[H - 1][W - 1] + 1, false);

  Worker curr(m.ini.first, m.ini.second, 0);

  map<pair<int, int>, vector<char>> bs;
  each (b, m.boosters) bs[{b.y, b.x}].push_back(b.c);

  wrap(m, curr);
  string t;
  while (true) {
    // clog << "current worker: " << curr.i << ' ' << curr.j << ' ' << curr.dir << endl;

    if (0 < curr.boosters['B'] && curr.pend.size()) {
      auto tmp = addManip(curr);
      auto p = make_pair(tmp.manips.back().i, tmp.manips.back().j);
      p.first += tmp.i;
      p.second += tmp.j;
      if (m.inside(p.first, p.second) && !m.blocked(p.first, p.second)) {
        curr = tmp;
        clog << curr.manips.back().j << ' ' <<  -curr.manips.back().i << endl;
        sprintf(buff, "B(%d,%d)", curr.manips.back().j, -curr.manips.back().i);
        t += string(buff);
        wrap(m, curr);
        continue;
      }
    }

    pair<int, char> mx = {-1, '@'};
    each (c, C) {
      Worker next = move(c, curr);
      auto w = wrappable(m, next);
      mx = max(mx, make_pair(w, c));
    }

    // clog << "mx: " << mx.first << ' ' << mx.second << ' ' << curr.manips.size() << endl;
    // return ;

    if (mx.first <= 0) {
      while (v.size() && colored[v.back().first][v.back().second]) v.pop_back();
      if (v.empty()) break;
      pair<int, int> dst = v.back();
      string s = bfs(make_pair(curr.i, curr.j), m);
      each (c, s) {
        if (colored[dst.first][dst.second]) break;
        curr = move(c, curr);
        wrap(m, curr);
        t += c;
        each (i, bs[make_pair(curr.i, curr.j)]) ++curr.boosters[i];
        bs[make_pair(curr.i, curr.j)].clear();
      }
    } else {
      curr = move(mx.second, curr);
      wrap(m, curr);
      t += mx.second;
      each (i, bs[make_pair(curr.i, curr.j)]) ++curr.boosters[i];
      bs[make_pair(curr.i, curr.j)].clear();

    }
  }
  cout << t << endl;
  clog << t.size() << endl;

  return ;
}
