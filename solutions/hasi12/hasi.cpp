#include "solve.hpp"

using namespace std;

pair<int,int> rotate(pair<int,int> v, int dir) {
	switch (dir) {
	case 0: return v;
	case 1: return {v.second, -v.first};
	case 2: return {-v.first, -v.second};
	case 3: return {-v.second, v.first};
	default: throw 1;
	}
}

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

struct Command {
	char c;
	bool twice;
	pair<int,int> mani, mani_out;
	Command(char c) : c(c), twice(false) {}
	Command(char c, bool twice) : c(c), twice(twice) {}
	Command(char c, pair<int,int> mani, pair<int,int> mani_out) : c(c), twice(false), mani(mani), mani_out(mani_out) {}
	string str() const {
		if (c == 'B') {
			ostringstream oss;
			oss << "B(" << mani_out.second << "," << -mani_out.first << ")";
			return oss.str();
		}
		return string() + c;
	}
};

bool operator<(const Command &lhs, const Command &rhs) {
	if (lhs.c == 'B' && rhs.c == 'B') {
		return lhs.mani < rhs.mani;
	} else {
		return lhs.c < rhs.c;
	}
}

struct Worker {
	int y, x, dir, fast, drill;
	vector<pair<int,int>> manipulators;
	vector<char> col_boosters;
	string result;
	Worker(int y, int x) : y(y), x(x), dir(0), fast(0), drill(0), manipulators({{-1,+1}, { 0,+1}, {+1,+1}}) {}
};

struct State {
	int H, W;
	vector<vector<bool>> board; // true: 進入可能
	map<pair<int,int>, vector<char>> boosters;
	set<pair<int,int>> spawn_points;
	vector<pair<int,int>> clones;

	vector<Worker> workers;
	map<char, int> cur_boosters;
	vector<vector<bool>> visited;
	vector<vector<bool>> rotten;
	int turn;

	State(const M& m, const string& buy) {
		H = m.grid.size();
		W = m.grid[0].size();
		board = vector<vector<bool>>(H, vector<bool>(W));
		for (int i = 0; i < H; ++ i) for (int j = 0; j < W; ++ j) {
			board[i][j] = (m.grid[i][j] != 0);
		}

		cerr << "buy = " << buy << endl;
		for (char c : buy) ++ cur_boosters[c];

		for (const auto& v : m.boosters) {
			switch (v.c) {
			case 'X':
				spawn_points.insert({v.y, v.x});
				break;
			case 'F':
			case 'L':
			case 'B':
			case 'C':
				boosters[{v.y, v.x}].push_back(v.c);
				if (v.c == 'C') {
					clones.push_back({v.y, v.x});
				}
				break;
			}
		}

		workers.push_back(Worker(m.ini.first, m.ini.second));

		visited = vector<vector<bool>>(H, vector<bool>(W));
		rotten = board;
		turn = 0;

		wrap(workers[0]);
	}

	void wrap(Worker& w) {
		board[w.y][w.x] = true;
		rotten[w.y][w.x] = false;
		for (const auto& v : w.manipulators) {
			auto vv = rotate(v, w.dir);
			int yy = w.y + vv.first;
			int xx = w.x + vv.second;
			if (0 <= yy && yy < H && 0 <= xx && xx < W && rotten[yy][xx]) {
				for (auto p : calc_obstacles(vv)) {
					if (!board[w.y + p.first][w.x + p.second]) goto next;
				}
				rotten[yy][xx] = false;
				next:;
			}
		}
		if (!visited[w.y][w.x]) {
			visited[w.y][w.x] = true;
			if (boosters.count({w.y, w.x})) {
				const auto& b = boosters[{w.y, w.x}];
				w.col_boosters.insert(w.col_boosters.end(), b.begin(), b.end());
			}
		}
	}

	bool check_finish() const {
		for (int i = 0; i < H; ++ i) for (int j = 0; j < W; ++ j) if (rotten[i][j]) return false;
		return true;
	}

	void input(int idx, const Command& cc) {
		auto& w = workers[idx];
		w.result += cc.str();

		for (char c : w.col_boosters) {
			++ cur_boosters[c];
		}
		w.col_boosters.clear();

		pair<int,int> c_pos;

		for (int t = 0; t < (cc.twice ? 2 : 1); ++ t) {
			switch (cc.c) {
			case 'W': -- w.y; break;
			case 'A': -- w.x; break;
			case 'S': ++ w.y; break;
			case 'D': ++ w.x; break;
			case 'E': w.dir = (w.dir + 1) % 4; break;
			case 'Q': w.dir = (w.dir + 3) % 4; break;
			case 'F': w.fast = 51; if(cur_boosters['F'] <= 0){cerr<<"booster error F"<<endl;throw 1;} -- cur_boosters['F']; break;
			case 'L': w.drill = 31; if(cur_boosters['L'] <= 0){cerr<<"booster error L"<<endl;throw 1;} -- cur_boosters['L']; break;
			case 'B': w.manipulators.push_back(cc.mani); if(cur_boosters['B'] <= 0){cerr<<"booster error B"<<endl;throw 1;} -- cur_boosters['B']; break;
			case 'C': c_pos = {w.y, w.x}; if(cur_boosters['C'] <= 0){cerr<<"booster error C"<<endl;throw 1;} -- cur_boosters['C']; break;
			default: cerr << "unknown command " << cc.c << endl; throw 1;
			}
			wrap(w);
		}
		if (w.fast > 0) -- w.fast;
		if (w.drill > 0) -- w.drill;

		if (cc.c == 'C') {
			workers.push_back(Worker(c_pos.first, c_pos.second));
		}
	}
};

struct Score {
	map<Command, map<int,int>> score1;
	map<int,int> top1;
	map<Command, int> score2;
	void set_score_1(const Command& c, int depth, int rotten_size, int col_booster_count) {
		score1[c][-depth] = max(score1[c][-depth], rotten_size + col_booster_count * 100);
	}
	void set_score_2(const Command& c, int rotten_dist) {
		score2[c] = max(score2[c], (1<<30) - rotten_dist);
	}
	map<Command, map<int,int>>::iterator get_top_1() {
		if (score1.empty()) {
			cerr << "score1 empty" << endl;
			throw 1;
		}
		
		auto top = score1.begin();
		int cnt = 1;
		auto it = top;
		++ it;
		for (; it != score1.end(); ++ it) {
			if (it->second > top->second) {
				top = it;
				cnt = 1;
			} else if (it->second == top->second) {
				if (rand() % ++ cnt) {
					top = it;
				}
			}
		}
		top1 = top->second;
		return cnt == 1 ? top : score1.end();
	}
	Command get_top_2() {
		if (score2.empty()) {
			cerr << "score2 empty" << endl;
			throw 1;
		}
		auto top = score2.begin();
		int cnt = 1;
		auto it = top;
		++ it;
		for (; it != score2.end(); ++ it) {
			if (it->second > top->second) {
				top = it;
				cnt = 1;
			} else if (it->second == top->second) {
				if (rand() % ++ cnt) {
					top = it;
				}
			}
		}
		return top->first;
	}
};

struct Solver {
	const State& state;
	Solver(const State& state) : state(state) {}

	map<char, int> num_boosters;
	vector<Command> cmds;
	Score score_obj;
	int mode;

	Command calc(int idx) {
		const auto& w = state.workers[idx];

		num_boosters = state.cur_boosters;
		for (char c : w.col_boosters) ++ num_boosters[c];

		if (num_boosters['C'] > 0 && state.spawn_points.count({w.y, w.x})) {
			return Command('C');
		}

		set<pair<int,int>> mani1(w.manipulators.begin(), w.manipulators.end());
		mani1.insert({0,0});

		for (mode = 1; mode <= 2; ++ mode) {
			if (mode == 1) {
				walk(3, w.y, w.x, w.dir, w.fast, w.drill, mani1, set<pair<int,int>>(), set<pair<int,int>>());
				auto it = score_obj.get_top_1();
				if (it != score_obj.score1.end()) {
					return it->first;
				}
			} else {
				walk(1, w.y, w.x, w.dir, w.fast, w.drill, mani1, set<pair<int,int>>(), set<pair<int,int>>());
				return score_obj.get_top_2();
			}
		}
		cerr << "not implemented" << endl;
		throw 1;
	}

	Command override(int idx, int i) {
		static const int dy[] = {-1, 0,+1, 0};
		static const int dx[] = { 0,-1, 0,+1};

		const auto& w = state.workers[idx];

		bool twice = false;
		int yy = w.y + dy[i];
		int xx = w.x + dx[i];
		if (!(0 <= yy && yy < state.H && 0 <= xx && xx < state.W)) { cerr << "invalid override" << endl; throw 1; }
		if (w.drill <= 0 && !state.board[yy][xx]) { cerr << "invalid override" << endl; throw 1; }
		if (w.fast > 0) {
			int yyy = yy + dy[i];
			int xxx = xx + dx[i];
			if (0 <= yyy && yyy < state.H && 0 <= xxx && xxx < state.W && (w.drill > 0 || state.board[yyy][xxx])) {
				twice = true;
			}
		}

		return Command("WASD"[i], twice);
	}

	void wrap(set<pair<int,int>>& rotten2, int y, int x, int dir, const set<pair<int,int>>& mani1, const set<pair<int,int>>& visited1) {
		for (const auto& v : mani1) {
			auto vv = rotate(v, dir);
			int yy = y + vv.first;
			int xx = x + vv.second;
			if (0 <= yy && yy < state.H && 0 <= xx && xx < state.W && state.rotten[yy][xx] && !rotten2.count({yy,xx})) {
				for (auto p : calc_obstacles(vv)) {
					if (!state.board[y + p.first][x + p.second] && !visited1.count({y + p.first, x + p.second})) goto next;
				}
				rotten2.insert({yy,xx});
				next:;
			}
		}
	}

	int calc_rotten_dist(int y0, int x0, int fast0, int drill0, const set<pair<int,int>>& rotten1) {
		static const int dy[] = {-1, 0,+1, 0};
		static const int dx[] = { 0,-1, 0,+1};

		queue<pair<int,int>> q;
		vector<vector<int>> dist(state.H, vector<int>(state.W, 1<<30));
		q.push({y0,x0});
		dist[y0][x0] = 0;
		while (!q.empty()) {
			int y = q.front().first;
			int x = q.front().second;
			int d = dist[y][x];
			bool fast = fast0 - d > 0;
			bool drill = drill0 - d > 0;
			q.pop();
			for (int i = 0; i < 4; ++ i) {
				int yy = y + dy[i];
				int xx = x + dx[i];
				if (!(0 <= yy && yy < state.H && 0 <= xx && xx < state.W)) continue;
				if (!drill && !state.board[yy][xx]) continue;
				if (fast) {
					int yyy = yy + dy[i];
					int xxx = xx + dx[i];
					if (0 <= yyy && yyy < state.H && 0 <= xxx && xxx < state.W && (drill || state.board[yyy][xxx])) {
						yy = yyy;
						xx = xxx;
					}
				}
				if (dist[yy][xx] > d+1) {
					if (state.rotten[yy][xx] && !rotten1.count({yy,xx})) {
						return d+1;
					}
					dist[yy][xx] = d+1;
					q.push({yy,xx});
				}
			}
		}
		return 0;
	}

	void walk(int depth, int y, int x, int dir, int fast, int drill, const set<pair<int,int>>& mani1, const set<pair<int,int>>& rotten1, const set<pair<int,int>>& visited1) {
		if (cmds.size() > 0) {
			if (mode == 1) {
				int col_booster_count = 0;
				for (auto p : visited1) {
					auto it = state.boosters.find(p);
					if (it != state.boosters.end()) {
						col_booster_count += it->second.size();
					}
				}
				score_obj.set_score_1(cmds[0], depth, rotten1.size(), col_booster_count);
			} else if (mode == 2) {
				if (score_obj.score1[cmds[0]] == score_obj.top1) {
					score_obj.set_score_2(cmds[0], calc_rotten_dist(y, x, fast, drill, rotten1));
				}
			} else {
				cerr << "invalid mode" << endl;
				throw 1;
			}
		}

		if (depth == 0) return;

		static const int dy[] = {-1, 0,+1, 0};
		static const int dx[] = { 0,-1, 0,+1};

		if (num_boosters.count('B') && num_boosters['B'] > 0) {
			-- num_boosters['B'];

			set<pair<int,int>> mani2;
			for (auto p : mani1) {
				for (int i = 0; i < 4; ++ i) {
					pair<int,int> v = {p.first + dy[i], p.second + dx[i]};
					if (!mani1.count(v) && mani2.insert(v).second) {
						cmds.push_back(Command('B', v, rotate(v, dir)));
						auto mani3 = mani1;
						mani3.insert(v);
						auto rotten2 = rotten1;
						wrap(rotten2, y, x, dir, mani3, visited1);
						walk(depth-1, y, x, dir, max(0, fast-1), max(0, drill-1), mani3, rotten1, visited1);
						cmds.pop_back();
					}
				}
			}

			++ num_boosters['B'];
			goto end;
		}
		for (int i = 0; i < 4; ++ i) {
			bool twice = false;
			int yy = y + dy[i];
			int xx = x + dx[i];
			if (!(0 <= yy && yy < state.H && 0 <= xx && xx < state.W)) continue;
			if (drill <= 0 && !state.board[yy][xx]) continue;
			auto rotten2 = rotten1;
			auto visited2 = visited1;
			if (!state.visited[yy][xx]) visited2.insert({yy,xx});
			wrap(rotten2, yy, xx, dir, mani1, visited2);
			if (fast > 0) {
				int yyy = yy + dy[i];
				int xxx = xx + dx[i];
				if (0 <= yyy && yyy < state.H && 0 <= xxx && xxx < state.W && (drill > 0 || state.board[yyy][xxx])) {
					twice = true;
					yy = yyy;
					xx = xxx;
					if (!state.visited[yy][xx]) visited2.insert({yy,xx});
					wrap(rotten2, yy, xx, dir, mani1, visited2);
				}
			}

			cmds.push_back(Command("WASD"[i], twice));
			walk(depth-1, yy, xx, dir, max(0, fast-1), max(0, drill-1), mani1, rotten2, visited2);
			cmds.pop_back();
		}
		for (int i = 0; i < 2; ++ i) {
			auto rotten2 = rotten1;
			int dir2 = (dir + 2*i+1) % 4;
			wrap(rotten2, y, x, dir2, mani1, visited1);
			cmds.push_back(Command("EQ"[i]));
			walk(depth-1, y, x, dir2, max(0, fast-1), max(0, drill-1), mani1, rotten2, visited1);
			cmds.pop_back();
		}
		if (num_boosters.count('F') && num_boosters['F'] > 0 && fast == 0) {
			-- num_boosters['F'];
			cmds.push_back(Command('F'));
			walk(depth-1, y, x, dir, 50, max(0, drill-1), mani1, rotten1, visited1);
			cmds.pop_back();
			++ num_boosters['F'];
		}
		if (num_boosters.count('L') && num_boosters['L'] > 0 && drill == 0) {
			-- num_boosters['L'];
			cmds.push_back(Command('L'));
			walk(depth-1, y, x, dir, max(0, fast-1), 30, mani1, rotten1, visited1);
			cmds.pop_back();
			++ num_boosters['L'];
		}
		end:;
	}
};

void bfs(const State& state, vector<vector<int>>& a, queue<pair<int,int>>& q) {
	static const int dy[] = {-1, 0,+1, 0};
	static const int dx[] = { 0,-1, 0,+1};

	while (!q.empty()) {
		auto p = q.front(); q.pop();
		int d = a[p.first][p.second];
		for (int i = 0; i < 4; ++ i) {
			int y = p.first + dy[i];
			int x = p.second + dx[i];
			if (0 <= y && y < state.H && 0 <= x && x < state.W && state.board[y][x] && d+1 < a[y][x]) {
				q.push({y,x});
				a[y][x] = d+1;
			}
		}
	}
}

void solve(const M& m, const string& buy, const char* filename) {
	static const int dy[] = {-1, 0,+1, 0};
	static const int dx[] = { 0,-1, 0,+1};

	State state(m, buy);

	vector<vector<int>> spawn_dist(state.H, vector<int>(state.W, 1<<30));
	queue<pair<int,int>> q;
	for (auto p : state.spawn_points) {
		q.push(p);
		spawn_dist[p.first][p.second] = 0;
	}
	bfs(state, spawn_dist, q);

	for (;;) {
		if (state.check_finish()) break;
		int n = state.workers.size();

		vector<bool> used(n);
		map<int, int> override;
		if (state.cur_boosters['C'] > 0) {
			vector<pair<int,int>> a;
			for (int idx = 0; idx < n; ++ idx) {
				const auto& w = state.workers[idx];
				a.push_back({spawn_dist[w.y][w.x], idx});
			}
			sort(a.begin(), a.end());
			int d = a[0].first;
			int idx = a[0].second;
			used[idx] = true;
			if (d > 0) {
				const auto& w = state.workers[idx];
				for (int i = 0; i < 4; ++ i) {
					int y = w.y + dy[i];
					int x = w.x + dx[i];
					if (0 <= y && y < state.H && 0 <= x && x < state.W && spawn_dist[y][x] == d-1) {
						override.insert({idx, i});
						break;
					}
				}
			}
		}

		vector<pair<int,pair<pair<int,int>,pair<int,int>>>> cln;
		for (auto p : state.clones) if (!state.visited[p.first][p.second]) {
			vector<vector<int>> a(state.H, vector<int>(state.W, 1<<30));
			queue<pair<int,int>> q;
			q.push(p);
			a[p.first][p.second] = 0;
			bfs(state, a, q);
			for (int idx = 0; idx < n; ++ idx) if (!used[idx]) {
				const auto& w = state.workers[idx];
				int d = a[w.y][w.x];
				for (int i = 0; i < 4; ++ i) {
					int y = w.y + dy[i];
					int x = w.x + dx[i];
					if (0 <= y && y < state.H && 0 <= x && x < state.W && a[y][x] == d-1) {
						cln.push_back({a[y][x], {p, {idx, i}}});
						break;
					}
				}
			}
		}
		sort(cln.begin(), cln.end());
		set<pair<int,int>> cln_used;
		for (auto v : cln) {
			auto p = v.second.first;
			int idx = v.second.second.first;
			int i = v.second.second.second;
			if (!used[idx]) if (cln_used.insert(p).second) {
				used[idx] = true;
				override.insert({idx, i});
			}
		}

		for (int idx = 0; idx < n; ++ idx) {
			auto it = override.find(idx);
			if (it != override.end()) {
				auto c = Solver(state).override(idx, it->second);
				state.input(idx, c);
			} else {
				//cerr << "calc start" << endl;
				auto c = Solver(state).calc(idx);
				//cerr << "calc end" << endl;
				//cerr << "input start" << endl;
				//cerr << "c = " << c.str() << endl;
				state.input(idx, c);
				//cerr << "input end" << endl;
			}
		}
		state.turn += 1;
	}

	for (unsigned idx = 0; idx < state.workers.size(); ++ idx) {
		if (idx) cout << "#";
		cout << state.workers[idx].result;
	}
	cout << endl;
	cerr << filename << " " << state.turn << endl;
}
