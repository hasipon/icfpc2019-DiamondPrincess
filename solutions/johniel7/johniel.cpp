#include "solve.hpp"

template<typename T> inline T setmax(T& a, T b) { return a = std::max(a, b); }
template<typename T> inline T setmin(T& a, T b) { return a = std::min(a, b); }

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

const int H = 450;
const int W = 450;

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

  int life;
  int fastWheel;

  Worker(int _i, int _j, int _dir)
  {
    fastWheel = 0;

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

  void boost(char c)
  {
    --boosters[c];
    switch (c) {
      case 'F':
        fastWheel = 50;
        break;
      default:
        assert("unknown booster");
    }
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

Worker move(Worker w, char c)
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

struct State {
  bool colored[H][W];
  State()
  {
    fill(&colored[0][0], &colored[H - 1][W - 1] + 1, false);
  }
};

int highCost[H][W];
void addWeight(int i, int j, int w, const State& s)
{
  if (!s.colored[i][j]) {
    highCost[i][j] += w;
  }
  int di[] = {-1, -1, -1,  0,  0, +1, +1, +1};
  int dj[] = {-1,  0, +1, -1, +1, -1,  0, +1};
  for (int d = 0; d < 4; ++d) {
    for (int a = 1; a < 5; ++a) {
      for (int b = 1; b < 5; ++b) {
        int ni = i + di[d] * a;
        int nj = j + dj[d] * b;
        unless (0 <= ni && ni < H) continue;
        unless (0 <= nj && nj < W) continue;
        if (!s.colored[i][j]) {
          highCost[i][j] += w / max(a, b);
        }
      }
    }
  }
  return ;
}

int wrappable(const M& m, const Worker& w, State& s)
{
  unless (m.inside(w.i, w.j)) return -(1 << 28);
  if (m.blocked(w.i, w.j)) return -(1 << 28);

  int cnt = 0;
  each (k, w.manips) {
    if (k.reachable(m, w.i, w.j)) {
      const int i = k.i + w.i;
      const int j = k.j + w.j;
      unless (0 <= i && i < H) continue;
      unless (0 <= j && j < W) continue;
      unless (s.colored[i][j]) {
        cnt += highCost[i][j];
        ++cnt;
      }
    }
  }
  assert(0 <= cnt);
  return cnt;
}

int wrap(const M& m, const Worker& w, State& s)
{
  int cnt = 0;
  each (k, w.manips) {
    if (k.reachable(m, w.i, w.j)) {
      ++cnt;
      s.colored[k.i + w.i][k.j + w.j] = true;
    }
  }
  return cnt;
}

bool isMoveOp(char c)
{
  return c == 'A' || c == 'D' || c == 'W' || c == 'S';
}

string bfs(pair<int, int> src, const M& m, State& state) // closest.
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
      if (!state.colored[ni][nj]) {
        dst = {ni, nj};
        break;
      }
    }
    if (dst != def) break;
  }
  if (dst == def) return "";
  pair<int, int> curr = dst;
  string t;
  while (src != curr) {
    t += cmd[curr];
    curr = path[curr.first][curr.second];
  }
  reverse(t.begin(), t.end());
  addWeight(dst.first, dst.second, t.size(), state);
  return t;
}
string bfs(int i, int j, const M& m, State& s)
{
  return bfs({i, j}, m, s);
}

char buff[100];
pair<int, string> hillClimbing(const string C, const M& m, Worker curr, State& state)
{
  map<pair<int, int>, vector<char>> bs;
  each (b, m.boosters) bs[{b.y, b.x}].push_back(b.c);

  wrap(m, curr, state);
  string t;
  while (true) {
    if (curr.boosters['B'] && curr.pend.size()) {
      auto tmp = addManip(curr);
      auto p = make_pair(tmp.manips.back().i, tmp.manips.back().j);
      p.first += tmp.i;
      p.second += tmp.j;
      if (m.inside(p.first, p.second) && !m.blocked(p.first, p.second)) {
        curr = tmp;
        sprintf(buff, "B(%d,%d)", curr.manips.back().j, -curr.manips.back().i);
        t += string(buff);
        wrap(m, curr, state);
        continue;
      }
    }

    if (false && curr.boosters['F']) {
      curr.boost('F');
      t += 'F';
      continue;
    }

    pair<int, char> mx = {-1, '@'};
    each (c, C) {
      const int rep = 1 + (isMoveOp(c) && curr.fastWheel);
      Worker tmp = curr;
      pair<int, char> p = {0, c};
      for (int j = 0; j < rep; ++j) {
        tmp = move(tmp, c);
        p.first += wrappable(m, tmp, state);
      }
      setmax(mx, p);
    }

    if (mx.first <= 0) {
      string s = bfs(curr.i, curr.j, m, state);
      if (s.empty()) break;
      each (c, s) {
        curr = move(curr, c);
        wrap(m, curr, state);
        t += c;
        each (i, bs[make_pair(curr.i, curr.j)]) ++curr.boosters[i];
        bs[make_pair(curr.i, curr.j)].clear();
      }
    } else {
      const int rep = 1 + (isMoveOp(mx.second) && curr.fastWheel);
      for (int j = 0; j < rep; ++j) {
        curr = move(curr, mx.second);
        wrap(m, curr, state);
        t += mx.second;
        each (i, bs[make_pair(curr.i, curr.j)]) ++curr.boosters[i];
        bs[make_pair(curr.i, curr.j)].clear();
      }
    }
  }
  return {t.size(), t};
}

void solve(const M& m)
{
  fill(&highCost[0][0], &highCost[H - 1][W - 1] + 1, 0);

  string C = "EQDSAW";
  pair<int, string> best = {1 << 29, ""};
  for (int _ = 0; _ < 80; ++_) {
    State state;
    Worker worker(m.ini.first, m.ini.second, 0);
    auto p = hillClimbing(C, m, worker, state);
    setmin(best, p);

    for (int i = 0; i < H; ++i) {
      for (int j = 0; j < W; ++j) {
        // highCost[i][j] *= 2;
        // highCost[i][j] /= 3;
      }
    }
    clog << _ << "-th generation: " << p.first << endl;
  }
  cout << best.second << endl;
  clog << best.first << endl;
  return ;
}
