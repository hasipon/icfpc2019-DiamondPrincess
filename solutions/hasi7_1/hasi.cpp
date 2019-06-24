#include "solve.hpp"

using namespace std;

const char* CMD = "WASDEQFL";

void move2(int& y, int& x, int& dir, int& fast, int& drill, map<char, int>& boosters, char c) {
	switch (c) {
	case 'W': -- y; break;
	case 'A': -- x; break;
	case 'S': ++ y; break;
	case 'D': ++ x; break;
	case 'E': dir = (dir + 1) % 4; break;
	case 'Q': dir = (dir + 3) % 4; break;
	case 'F': fast = 51; if(boosters['F'] <= 0){cerr<<"booster error F"<<endl;throw 1;} -- boosters['F']; break;
	case 'L': drill = 31; if(boosters['L'] <= 0){cerr<<"booster error L"<<endl;throw 1;} -- boosters['L']; break;
	case 'B': if(boosters['B'] <= 0){cerr<<"booster error B"<<endl;throw 1;} -- boosters['B']; break;
	case 'C': if(boosters['C'] <= 0){cerr<<"booster error C"<<endl;throw 1;} -- boosters['C']; break;
	default: throw 1;
	}
}

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
		// cerr << x1 << " " << x2 << " ; " << (2*y-1)*dx+dy << "/" << 2*dy << " " << (2*y+1)*dx+dy << "/" << 2*dy << endl;
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
	int mode;
	int y, x, dir, fast, drill;
	vector<pair<int,int>> manipulators;
	pair<int,int> target;
	string result;
	Worker(int y, int x) : mode(0), y(y), x(x), dir(0), fast(0), drill(0), manipulators({{-1,+1}, { 0,+1}, {+1,+1}}) {}
};

struct State {
	int H, W;
	vector<vector<bool>> board_init; // true: 進入可能
	map<pair<int,int>, vector<char>> boosters;
	vector<pair<int,int>> clones;
	set<pair<int,int>> spawn_points;

	vector<Worker> workers;
	map<int,map<char, int>> col_boosters;
	map<char, int> cur_boosters;
	vector<vector<bool>> visited;
	vector<vector<bool>> rotten;
	int turn;

	State(const M& m) {
		H = m.grid.size();
		W = m.grid[0].size();
		board_init = vector<vector<bool>>(H, vector<bool>(W));
		for (int i = 0; i < H; ++ i) for (int j = 0; j < W; ++ j) {
			board_init[i][j] = (m.grid[i][j] != 0);
		}

		for (const auto& v : m.boosters) {
			switch (v.c) {
			case 'X':
				spawn_points.insert({v.y, v.x});
				break;
			default:
				boosters[{v.y, v.x}].push_back(v.c);
				if (v.c == 'C') {
					clones.push_back({v.y, v.x});
				}
			}
		}

		workers.push_back(Worker(m.ini.first, m.ini.second));

		visited = vector<vector<bool>>(H, vector<bool>(W));
		rotten = board_init;
		turn = 0;

		wrap(0);
	}

	void wrap(int idx) {
		const auto& w = workers[idx];
		board_init[w.y][w.x] = true;
		rotten[w.y][w.x] = false;
		for (const auto& v : w.manipulators) {
			auto vv = rotate(v, w.dir);
			int yy = w.y + vv.first;
			int xx = w.x + vv.second;
			if (0 <= yy && yy < H && 0 <= xx && xx < W) {
				for (auto p : calc_obstacles(vv)) {
					if (!board_init[w.y + p.first][w.x + p.second]) goto next;
				}
				rotten[yy][xx] = false;
				next:;
			}
		}
		if (!visited[w.y][w.x]) {
			visited[w.y][w.x] = true;
			if (boosters.count({w.y, w.x})) {
				for (char v : boosters[{w.y, w.x}]) {
					++ col_boosters[idx][v];
				}
			}
		}
	}

	bool check_finish() const {
		for (int i = 0; i < H; ++ i) for (int j = 0; j < W; ++ j) if (rotten[i][j]) return false;
		return true;
	}

	void input(const vector<Command>& commands) {
		if (commands.size() != workers.size()) { cerr << "invalid commands size" << endl; throw 1; }
		turn += 1;
		// for (unsigned idx = 0; idx < commands.size(); ++ idx) cerr << commands[idx].str() << " "; cerr << endl;
		for (unsigned idx = 0; idx < commands.size(); ++ idx) {
			auto& w = workers[idx];
			const auto& cc = commands[idx];
			if (col_boosters.count(idx)) {
				for (auto p : col_boosters[idx]) {
					cur_boosters[p.first] += p.second;
				}
			}
			col_boosters.erase(idx);
			w.result += cc.str();
			if (cc.c == 'B') {
				w.manipulators.push_back(cc.mani);
			} else if (cc.c == 'C') {
				workers.push_back(Worker(w.y, w.x));
			}
			move2(w.y, w.x, w.dir, w.fast, w.drill, cur_boosters, cc.c);
			wrap(idx);
			if (cc.twice) {
				move2(w.y, w.x, w.dir, w.fast, w.drill, cur_boosters, cc.c);
				wrap(idx);
			}
			if (w.fast > 0) -- w.fast;
			if (w.drill > 0) -- w.drill;
		}
	}
};

struct Solver {
	const State& state;

	map<char, int> s_boosters;
	set<pair<int,int>> s_diff_rotten;

	Solver(const State& state) : state(state) {
		s_boosters = state.cur_boosters;
	}

	int sel_idx;
	vector<Command> cmds;
	vector<vector<Command>> valid_cmds;

	Command calc(int idx, int depth) {
		const auto& w = state.workers[idx];
		if (state.col_boosters.count(idx)) {
			for (auto p : state.col_boosters.find(idx)->second) {
				s_boosters[p.first] += p.second;
			}
		}
		if (s_boosters['C'] > 0 && state.spawn_points.count({w.y, w.x})) {
			-- s_boosters['C'];
			return Command('C');
		}
		if (w.mode == 1 || w.mode == 2) {
			vector<vector<int>> dist(state.H, vector<int>(state.W, 1<<30));
			queue<pair<int,int>> q;
			if (w.mode == 1) {
				dist[w.target.first][w.target.second] = 0;
				q.push(w.target);
			} else {
				for (auto p : state.spawn_points) {
					dist[p.first][p.second] = 0;
					q.push(p);
				}
			}
			while (!q.empty()) {
				int y = q.front().first;
				int x = q.front().second;
				q.pop();
				static const int dy[] = {-1, 0,+1, 0};
				static const int dx[] = { 0,-1, 0,+1};
				for (int i = 0; i < 4; ++ i) {
					int yy = y + dy[i];
					int xx = x + dx[i];
					if (0 <= yy && yy < state.H && 0 <= xx && xx < state.W && state.board_init[yy][xx]) {
						if (dist[y][x] + 1 < dist[yy][xx]) {
							dist[yy][xx] = dist[y][x] + 1;
							q.push({yy,xx});
						}
					}
				}
			}
			int d = 1<<30;
			char sel;
			for (int i = 0; i < 4; ++ i) {
				int y = w.y;
				int x = w.x;
				switch (CMD[i]) {
				case 'W': -- y; break;
				case 'A': -- x; break;
				case 'S': ++ y; break;
				case 'D': ++ x; break;
				}
				if (0 <= y && y < state.H && 0 <= x && x < state.W && dist[y][x] < d) {
					d = dist[y][x];
					sel = CMD[i];
				}
			}
			return Command(sel);
		} else {
			sel_idx = idx;
			cmds.clear();
			valid_cmds.clear();
			walk(depth);
			auto c = choose();
			if (s_boosters.count(c.c)) {
				-- s_boosters[c.c];
			}
			return c;
		}
	}
	bool is_valid() {
		const auto& w = state.workers[sel_idx];
		int cur_y = w.y;
		int cur_x = w.x;
		int cur_dir = w.dir;
		int cur_fast = w.fast;
		int cur_drill = w.drill;
		auto cur_boosters = s_boosters;
		for (const auto& c : cmds) {
			if (c.twice && cur_fast == 0) return false;
			if (c.c == 'F' && (cur_boosters['F'] <= 0 || cur_fast > 0)) return false;
			if (c.c == 'L' && (cur_boosters['L'] <= 0 || cur_drill > 0)) return false;
			move2(cur_y, cur_x, cur_dir, cur_fast, cur_drill, cur_boosters, c.c);
			if (!(0 <= cur_y && cur_y < state.H && 0 <= cur_x && cur_x < state.W)) return false;
			if (cur_drill <= 0 && !state.board_init[cur_y][cur_x]) return false;
			if (c.twice) {
				move2(cur_y, cur_x, cur_dir, cur_fast, cur_drill, cur_boosters, c.c);
				if (!(0 <= cur_y && cur_y < state.H && 0 <= cur_x && cur_x < state.W)) return false;
				if (cur_drill <= 0 && !state.board_init[cur_y][cur_x]) return false;
			}

			if (cur_fast > 0) -- cur_fast;
			if (cur_drill > 0) -- cur_drill;
		}
		return true;
	}
	void walk(int depth) {
		for (int i = 0; i < 4; ++ i) {
			cmds.push_back(Command(CMD[i], true));
			if (is_valid()) {
				valid_cmds.push_back(cmds);
				if (depth > 1) walk(depth-1);
			} else {
				cmds.pop_back();
				cmds.push_back(Command(CMD[i], false));
				if (is_valid()) {
					valid_cmds.push_back(cmds);
					if (depth > 1) walk(depth-1);
				}
			}
			cmds.pop_back();
		}
		for (int i = 4; i < 8; ++ i) {
			auto c = Command(CMD[i]);
			cmds.push_back(c);
			if (is_valid()) {
				valid_cmds.push_back(cmds);
				if (depth > 1) walk(depth-1);
			}
			cmds.pop_back();
		}

		int num_mani = s_boosters['B'];
		const auto& w = state.workers[sel_idx];
		set<pair<int,int>> mani1(w.manipulators.begin(), w.manipulators.end());
		mani1.insert({0,0});
		int dir = w.dir;
		for (auto c : cmds) {
			switch (c.c) {
			case 'B': -- num_mani; break;
			case 'E': dir = (dir + 1) % 4; break;
			case 'Q': dir = (dir + 3) % 4; break;
			}
		}
		if (num_mani > 0) {
			static const int dy[] = {-1, 0,+1, 0};
			static const int dx[] = { 0,-1, 0,+1};

			set<pair<int,int>> mani2;
			for (auto p : mani1) {
				for (int i = 0; i < 4; ++ i) {
					int yy = p.first + dy[i];
					int xx = p.second + dx[i];
					if (!mani1.count({yy,xx})) {
						mani2.insert({yy,xx});
					}
				}
			}
			for (auto v : mani2) {
				auto c = Command('B', v, rotate(v, dir));
				cmds.push_back(c);
				if (is_valid()) {
					valid_cmds.push_back(cmds);
					if (depth > 1) walk(depth-1);
				}
				cmds.pop_back();
			}
		}
	}
	int calc_score1(const vector<Command>& cc) const {
		const auto& w = state.workers[sel_idx];
		int cur_y = w.y;
		int cur_x = w.x;
		int cur_dir = w.dir;
		int cur_fast = w.fast;
		int cur_drill = w.drill;
		auto cur_boosters = s_boosters;
		auto diff_rotten = s_diff_rotten;
		int num_mani = 0;
		set<pair<int,int>> diff_visisted;
		int col_booster_count = 0;
		for (const auto& c : cc) {
			if (c.c == 'B') ++ num_mani;
			for (int t = 0; t < (c.twice ? 2 : 1); ++ t) {
				move2(cur_y, cur_x, cur_dir, cur_fast, cur_drill, cur_boosters, c.c);
				if (!state.visited[cur_y][cur_x]) {
					if (diff_visisted.insert({cur_y, cur_x}).second) {
						auto it = state.boosters.find({cur_y, cur_x});
						if (it != state.boosters.end()) {
							col_booster_count += it->second.size();
						}
					}
				}

				if (state.rotten[cur_y][cur_x]) diff_rotten.insert({cur_y, cur_x});
				for (const auto& v : w.manipulators) {
					auto vv = rotate(v, cur_dir);
					int yy = cur_y + vv.first;
					int xx = cur_x + vv.second;
					if (0 <= yy && yy < state.H && 0 <= xx && xx < state.W && state.rotten[yy][xx]) {
						for (auto p : calc_obstacles(vv)) {
							if (!state.board_init[cur_y + p.first][cur_x + p.second]) goto next;
						}
						diff_rotten.insert({yy, xx});
						next:;
					}
				}
			}
		}
		return (int)diff_rotten.size() + col_booster_count * 100 + num_mani * 10000;
	}

	int calc_score2(const Command& c) const {
		const auto& w = state.workers[sel_idx];
		int cur_y = w.y;
		int cur_x = w.x;
		int cur_dir = w.dir;
		int cur_fast = w.fast;
		int cur_drill = w.drill;
		auto cur_boosters = s_boosters;
		set<pair<int,int>> diff_rotten;
		for (int t = 0; t < (c.twice ? 2 : 1); ++ t) {
			move2(cur_y, cur_x, cur_dir, cur_fast, cur_drill, cur_boosters, c.c);

			if (state.rotten[cur_y][cur_x]) diff_rotten.insert({cur_y, cur_x});
			for (const auto& v : w.manipulators) {
				auto vv = rotate(v, cur_dir);
				int yy = cur_y + vv.first;
				int xx = cur_x + vv.second;
				if (0 <= yy && yy < state.H && 0 <= xx && xx < state.W && state.rotten[yy][xx]) {
					for (auto p : calc_obstacles(vv)) {
						if (!state.board_init[cur_y + p.first][cur_x + p.second]) goto next;
					}
					diff_rotten.insert({yy, xx});
					next:;
				}
			}
		}
		if (cur_fast > 0) -- cur_fast;
		if (cur_drill > 0) -- cur_drill;

		static const int dy[] = {-1, 0,+1, 0};
		static const int dx[] = { 0,-1, 0,+1};

		set<pair<int,int>> visited;
		visited.insert({cur_y, cur_x});
		queue<pair<pair<int,int>,int>> q;
		q.push({{cur_y, cur_x},0});
		while (!q.empty()) {
			int y = q.front().first.first;
			int x = q.front().first.second;
			int dist = q.front().second;
			q.pop();
			for (int i = 0; i < 4; ++ i) {
				int yy = y + dy[i];
				int xx = x + dx[i];
				if (0 <= yy && yy < state.H && 0 <= xx && xx < state.W && (cur_drill - dist > 0 || state.board_init[yy][xx])) {
					if (cur_fast - dist > 0) {
						int yyy = yy + dy[i];
						int xxx = xx + dx[i];
						if (0 <= yyy && yyy < state.H && 0 <= xxx && xxx < state.W && (cur_drill - dist > 0 || state.board_init[yyy][xxx])) {
							yy = yyy;
							xx = xxx;
						}
					}
					if (state.board_init[yy][xx] && state.rotten[yy][xx] && !diff_rotten.count({yy,xx})) return -dist;

					if (!visited.count({yy, xx})) {
						visited.insert({yy,xx});
						q.push({{yy,xx},dist+1});
					}
				}
			}
		}
		return 0;
	}
	Command choose() const {
		map<Command,map<int,int>> score1;
		for (const auto& c : valid_cmds) {
			int s = calc_score1(c);
			//string debug; for (auto v : c) {debug+=v.c; if(v.twice)debug+="2";}cerr << debug << " " << s << endl;
			if (!score1[c[0]].count(c.size()) || s > score1[c[0]][c.size()]) {
				score1[c[0]][c.size()] = s;
			}
		}
		set<Command> fst1;
		vector<int> max_score1;
		for (const auto& p : score1) {
			vector<int> s;
			for (const auto& q : p.second) {
				s.push_back(q.second);
			}
			reverse(s.begin(), s.end());
			if (max_score1.empty() || s > max_score1) {
				max_score1 = s;
				fst1 = {p.first};
			} else if (s == max_score1) {
				fst1.insert(p.first);
			}
		}
		if (fst1.empty()) { cerr << "fst1 empty" << endl; throw 1; }
		if (fst1.size() == 1) return *fst1.begin();

		vector<Command> fst2;
		int max_score2 = -(1<<30);
		for (const auto& c : fst1) {
			int s = calc_score2(c);
			//cerr << c.c << " " << s << endl;
			if (s > max_score2) {
				max_score2 = s;
				fst2 = {c};
			} else if (s == max_score2) {
				fst2.push_back(c);
			}
		}
		//cerr << state.result << endl;
		//cerr << state.cur_drill << endl;
		//cerr << endl;
		if (fst2.empty()) { cerr << "fst2 empty" << endl; throw 1; }
		return fst2[rand() % fst2.size()];
	}
};

void solve(const M& m) {
	State inst(m);

	for (;;) {
		if (inst.check_finish()) break;
		Solver solver(inst);
		vector<Command> commands;
		for (unsigned idx = 0; idx < inst.workers.size(); ++ idx) {
			auto& w = inst.workers[idx];
			if (w.mode == 0) {
				if (inst.clones.size() > 0) {
					w.mode = 1;
					w.target = inst.clones.back(); inst.clones.pop_back();
				} else {
					w.mode = 3;
				}
			} else if (w.mode == 1) {
				if (w.target == make_pair(w.y, w.x)) {
					w.mode = 2;
				}
			}
			auto c = solver.calc(idx, 3);
			commands.push_back(c);
			if (c.c == 'C') {
				w.mode = 0;
			}
		}
		inst.input(commands);
	}

	for (unsigned idx = 0; idx < inst.workers.size(); ++ idx) {
		if (idx) cout << "#";
		cout << inst.workers[idx].result;
	}
	cout << endl;
	//cerr << inst.turn << endl;
}
