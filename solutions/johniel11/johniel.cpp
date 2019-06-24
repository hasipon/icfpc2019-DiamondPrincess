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
  queue<char> schedule;

  int fastWheel;

  string cmd;
  bool ready;

  Worker(int _i, int _j, int _dir)
  {
    fastWheel = 0;
    ready = true;

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
    switch (c) {
      case 'F':
        fastWheel = 51;
        break;
      default:
        assert("unknown booster");
    }
  }
};

struct State {
  bool colored[H][W];
  vector<Worker> workers;
  map<char, int> boosters;
  vector<pair<int, int>> x;

  State(const M& m, string buy)
  {
    fill(&colored[0][0], &colored[H - 1][W - 1] + 1, false);
    workers.push_back(Worker(m.ini.first, m.ini.second, 0));
    each (b, m.boosters) if (b.c == 'X') {
      x.push_back({b.y, b.x});
    }

    each (c, buy) ++boosters[c];
  }

  void spawn(const Worker& parent)
  {
    workers.push_back(Worker(parent.i, parent.j, 0));
    return ;
  }
};

Worker addManip(Worker w, State& s)
{
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
    case 'Z': break;
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

int highCost[H][W]; // 前世の記憶
void addWeight(int i, int j, int w, const State& s)
{
  if (1 || !s.colored[i][j]) {
    highCost[i][j] += w;
  }
  int di[] = {-1, -1, -1,  0,  0, +1, +1, +1};
  int dj[] = {-1,  0, +1, -1, +1, -1,  0, +1};
  for (int d = 0; d < 4; ++d) {
    for (int a = 1; a < 2; ++a) {
      for (int b = 1; b < 2; ++b) {
        int ni = i + di[d] * a;
        int nj = j + dj[d] * b;
        unless (0 <= ni && ni < H) continue;
        unless (0 <= nj && nj < W) continue;
        if (1 || !s.colored[i][j]) {
          highCost[i][j] += w / max(a, b);
          // highCost[i][j] += w;
        }
      }
    }
  }
  return ;
}

int wrappable(const M& m, const Worker& w, State& s)
{
  unless (m.inside(w.i, w.j)) return -(1 << 27);
  if (m.blocked(w.i, w.j)) return -(1 << 27);
  if (!w.ready) return 0;

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

int wrap(const M& m, State& s)
{
  int cnt = 0;
  each (w, s.workers) {
    if (!w.ready) continue;
    each (k, w.manips) {
      if (k.reachable(m, w.i, w.j)) {
        ++cnt;
        s.colored[k.i + w.i][k.j + w.j] = true;
      }
    }
  }
  return cnt;
}

bool isMoveOp(char c)
{
  return c == 'A' || c == 'D' || c == 'W' || c == 'S';
}

string nearest(const Worker& w, const M& m, const State& state, const pair<int, int> G = make_pair(-1, -1))
{
  static map<tuple<int, int, int>, tuple<int, int, int>> path;
  static map<tuple<int, int, int>, char> cmd;
  map<tuple<int, int, int>, int> cost;
  cost[{w.i, w.j, w.fastWheel}] = 0;

  const int di[] = {0, 0, -1, +1, 0};
  const int dj[] = {-1, +1, 0, 0, 0};
  const string C = "ADWSZ";

  constexpr tuple<int, int, int> def = {-1, -1, -1};
  tuple<int, int, int> dst = def;
  priority_queue<pair<int, tuple<int, int, int>>> q;
  q.push({0, {w.i, w.j, w.fastWheel}});
  while (!q.empty()) {
    const int c = -q.top().first;
    int i, j, fast;
    tie(i, j, fast) = q.top().second;
    q.pop();
    if (cost[{i, j, fast}] != c) continue;
    for (int d = 0; d < C.size(); ++d) {
      bool f = false;
      int ni = i;
      int nj = j;
      const int nf = max(0, fast - 1);
      for (int len = 0; len < (1 + !!fast); ++len) {
        ni += di[d];
        nj += dj[d];
        f = false;
        unless (m.inside(ni, nj)) break;
        if (m.blocked(ni, nj)) break;
        f = true;
      }
      if (!f) continue;
      if (cost.count({ni, nj, nf}) && cost[{ni, nj, nf}] <= c + 1) continue;
      cost[{ni, nj, nf}] = c + 1;
      q.push({-(c + 1), {ni, nj, nf}});
      path[{ni, nj, nf}] = {i, j, fast};
      cmd[{ni, nj, nf}] = C[d];
      if (!state.colored[ni][nj] && G == make_pair(-1, -1)) {
        dst = {ni, nj, nf};
        break;
      }
      if (G == make_pair(ni, nj)) {
        dst = {ni, nj, nf};
        break;
      }
    }
    if (dst != def) break;
  }
  if (dst == def) {
    for (int i = 0; i < m.h(); ++i) {
      for (int j = 0; j < m.w(); ++j) {
        if (m.blocked(i, j)) continue;
        if (!state.colored[i][j]) {
          cerr << "G: "<< G.first << ' ' << G.second << endl;
          cerr << i << ' ' << j << " is not colored" << endl;
          cerr << "Worker: " << w.i << ' ' << w.j << ' ' << w.fastWheel << endl;
          cerr << cost.size() << endl;
          assert(state.colored[i][j]);
        }
      }
    }
    return "";
  }
  const tuple<int, int, int> src = {w.i, w.j, w.fastWheel};
  tuple<int, int, int> curr = dst;
  string t;
  while (src != curr) {
    assert(!m.blocked(get<0>(curr), get<1>(curr)));
    t += cmd[curr];
    curr = path[curr];
  }
  reverse(t.begin(), t.end());
  {
    int i, j, f;
    tie(i, j, f) = dst;
    addWeight(i, j, t.size(), state);
  }
  return t;
}

pair<int, string> hillClimbing(const string C, const M& m, State& state)
{
  set<pair<int, int>> X;
  map<char, vector<pair<int, int>>> boostpos;

  auto fn = [&] (Worker& w, int idx) {
    if (!w.ready) return true;

    if (!w.schedule.empty()) {
      char c = w.schedule.front();
      w.schedule.pop();
      const int rep = 1 + (isMoveOp(c) && w.fastWheel);
      w.cmd += c;
      for (int j = 0; j < rep; ++j) {
        w = move(w, c);
        wrap(m, state);
      }
      return true;
    }

    if (state.boosters['C'] && X.count({w.i, w.j})) {
      --state.boosters['C'];
      w.cmd += 'C';
      state.spawn(w);
      return true;
    }

    if (idx == 0) {
      string s;
      if (state.boosters['C'] == 0 && boostpos['C'].size()) {
        sort(boostpos['C'].begin(), boostpos['C'].end(), [&] (auto a, auto b) {
          int p = abs(a.first - w.i) + abs(a.second - w.j);
          int q = abs(b.first - w.i) + abs(b.second - w.j);
          return p > q;
        });
        s = nearest(w, m, state, boostpos['C'].back());
        boostpos['C'].pop_back();
        assert(s.size());
      }
      if (state.boosters['C'] && X.size()) {
        sort(boostpos['X'].begin(), boostpos['X'].end(), [&] (auto a, auto b) {
          int p = abs(a.first - w.i) + abs(a.second - w.j);
          int q = abs(b.first - w.i) + abs(b.second - w.j);
          return p > q;
        });
        s = nearest(w, m, state, boostpos['X'].back());
        assert(s.size());
      }
      if (s.size()) {
        each (c, s) w.schedule.push(c);
        char c = w.schedule.front();
        w.schedule.pop();
        const int rep = 1 + (isMoveOp(c) && w.fastWheel);
        w.cmd += c;
        for (int j = 0; j < rep; ++j) {
          w = move(w, c);
          wrap(m, state);
        }
        return true;
      }
    }

    if (state.boosters['B'] && w.pend.size()) {
      auto tmp = addManip(w, state);
      auto p = make_pair(tmp.manips.back().i, tmp.manips.back().j);
      p.first += tmp.i;
      p.second += tmp.j;
      if (m.inside(p.first, p.second) && !m.blocked(p.first, p.second)) {
        w = tmp;
        static char buff[100];
        sprintf(buff, "B(%d,%d)", w.manips.back().j, -w.manips.back().i);
        w.cmd += string(buff);
        --state.boosters['B'];
        wrap(m, state);
        return true;
      }
    }

    if (state.boosters['F'] && !w.fastWheel) {
      --state.boosters['F'];
      w.boost('F');
      w.cmd += 'F';
      return true;
    }

    pair<int, char> mx = {-1, '@'};
    each (c, C) {
      const int rep = 1 + (isMoveOp(c) && w.fastWheel);
      Worker tmp = w;
      pair<int, char> p = {0, c};
      for (int j = 0; j < rep; ++j) {
        tmp = move(tmp, c);
        p.first += wrappable(m, tmp, state);
      }
      if (mx.first < p.first) {
        mx = p;
      }
    }

    if (mx.first <= 0) {
      string s = nearest(w, m, state);
      if (s.empty()) return false;
      each (c, s) w.schedule.push(c);
      mx.second = w.schedule.front();
      w.schedule.pop();
    }
    const int rep = 1 + (isMoveOp(mx.second) && w.fastWheel);
    w.cmd += mx.second;
    for (int j = 0; j < rep; ++j) {
      w = move(w, mx.second);
      wrap(m, state);
    }
    return true;
  };

  map<pair<int, int>, vector<char>> bs;
  each (b, m.boosters) {
    addWeight(b.y, b.x, 100, state);
    bs[{b.y, b.x}].push_back(b.c);
    if (b.c == 'X') X.insert({b.y, b.x});
    boostpos[b.c].push_back({b.y, b.x});
  }

  each (w, state.workers) w.ready = true;
  wrap(m, state);
  while (true) {
    bool f = true;
    for (int i = 0, j = state.workers.size(); i < j; ++i) {
      Worker& w = state.workers[i];
      if (!w.ready) continue;
      each (k, bs[make_pair(w.i, w.j)]) ++state.boosters[k];
      bs[make_pair(w.i, w.j)].clear();
      f = f && fn(w, i);
    }
    unless (f) break;
    each (w, state.workers) w.fastWheel = max(0, w.fastWheel - 1);
    each (w, state.workers) w.ready = true;
  }

  int c = state.workers.front().cmd.size();
  string t;
  each (w, state.workers) {
    if (!t.empty()) t += '#';
    t += w.cmd;
  }
  return {c, t};
}

void solve(const M& m, const string& buy, const char* filename)
{
  fill(&highCost[0][0], &highCost[H - 1][W - 1] + 1, 0);

  string C = "EQDSAWZ";
  pair<int, string> best = {1 << 29, ""};

  if (100 < max(m.h(), m.w())) {
    for (int _ = 0; _ < 35; ++_) {
      random_shuffle(C.begin(), C.end());
      State state(m, buy);
      auto p = hillClimbing(C, m, state);
      setmin(best, p);
      clog << C << ": " << p.first << endl;
    } while (next_permutation(C.begin(), C.end()));
  } else {
    sort(C.begin(), C.end());
    int cnt = 0;
    do {
      if (++cnt % 6) continue;
      State state(m, buy);
      auto p = hillClimbing(C, m, state);
      setmin(best, p);
      clog << C << ": " << p.first << endl;
    } while (next_permutation(C.begin(), C.end()));
  }

  cout << best.second << endl;
  clog << best.first << endl;

  return ;
}
