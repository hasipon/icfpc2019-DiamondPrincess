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
	case 'F': fast = 51; -- boosters['F']; break;
	case 'L': drill = 31; -- boosters['L']; break;
	case 'B': -- boosters['B']; break;
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
    return lhs.c < rhs.c;
}

struct State {
	int H, W;
	vector<vector<bool>> board_init; // true: 進入可能
	map<pair<int,int>, vector<char>> boosters;

	string result;
	int cur_y, cur_x, cur_dir, cur_fast, cur_drill;
	map<char, int> cur_boosters;
	vector<vector<bool>> visited;
	vector<vector<bool>> rotten;
	vector<pair<int,int>> manipulators;
	int turn;

	State(const M& m) {
		H = m.grid.size();
		W = m.grid[0].size();
		board_init = vector<vector<bool>>(H, vector<bool>(W));
		for (int i = 0; i < H; ++ i) for (int j = 0; j < W; ++ j) {
			board_init[i][j] = (m.grid[i][j] != 0);
		}

		for (const auto& v : m.boosters) {
			boosters[{v.y, v.x}].push_back(v.c);
		}

		cur_y = m.ini.first;
		cur_x = m.ini.second;
		cur_dir = 0;
		visited = vector<vector<bool>>(H, vector<bool>(W));
		rotten = board_init;
		manipulators = {{-1,+1}, { 0,+1}, {+1,+1}};
		turn = 0;

		wrap();
	}

	void wrap() {
		board_init[cur_y][cur_x] = true;
		rotten[cur_y][cur_x] = false;
		for (const auto& v : manipulators) {
			auto vv = rotate(v, cur_dir);
			int yy = cur_y + vv.first;
			int xx = cur_x + vv.second;
			if (0 <= yy && yy < H && 0 <= xx && xx < W) {
				for (auto p : calc_obstacles(vv)) {
					if (!board_init[cur_y + p.first][cur_x + p.second]) goto next;
				}
				rotten[yy][xx] = false;
				next:;
			}
		}
		if (!visited[cur_y][cur_x]) {
			visited[cur_y][cur_x] = true;
			if (boosters.count({cur_y, cur_x})) {
				for (char v : boosters[{cur_y, cur_x}]) {
					++ cur_boosters[v];
				}
			}
		}
	}

	bool check_finish() const {
		for (int i = 0; i < H; ++ i) for (int j = 0; j < W; ++ j) if (rotten[i][j]) return false;
		return true;
	}

	int get_boosters(char c) const {
		auto it = cur_boosters.find(c);
		if (it == cur_boosters.end()) return 0;
		return it->second;
	}

	void input(const Command cc) {
		// cerr << cur_y << " " << cur_x << " " << cur_dir << " " << cc.str() << endl;
		result += cc.str();
		turn += 1;
		if (cc.c == 'B') {
			manipulators.push_back(cc.mani);
		}
		move2(cur_y, cur_x, cur_dir, cur_fast, cur_drill, cur_boosters, cc.c);
		wrap();
		if (cc.twice) {
			move2(cur_y, cur_x, cur_dir, cur_fast, cur_drill, cur_boosters, cc.c);
			wrap();
		}
		if (cur_fast > 0) -- cur_fast;
		if (cur_drill > 0) -- cur_drill;
	}
};

bool is_valid(const State& state, const vector<Command>& cmds) {
	int cur_y = state.cur_y;
	int cur_x = state.cur_x;
	int cur_dir = state.cur_dir;
	int cur_fast = state.cur_fast;
	int cur_drill = state.cur_drill;
	auto cur_boosters = state.cur_boosters;
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

struct Solver {
	const State& state;
	vector<Command> cmds;
	vector<vector<Command>> valid_cmds;
	Solver(const State& state, int depth) : state(state) {
		walk(depth);
	}

	void walk(int depth) {
		for (int i = 0; i < 4; ++ i) {
			cmds.push_back(Command(CMD[i], true));
			if (is_valid(state, cmds)) {
				valid_cmds.push_back(cmds);
				if (depth > 1) walk(depth-1);
			} else {
				cmds.pop_back();
				cmds.push_back(Command(CMD[i], false));
				if (is_valid(state, cmds)) {
					valid_cmds.push_back(cmds);
					if (depth > 1) walk(depth-1);
				}
			}
			cmds.pop_back();
		}
		for (int i = 4; i < 8; ++ i) {
			auto c = Command(CMD[i]);
			cmds.push_back(c);
			if (is_valid(state, cmds)) {
				valid_cmds.push_back(cmds);
				if (depth > 1) walk(depth-1);
			}
			cmds.pop_back();
		}

		int num_mani = state.get_boosters('B');
		set<pair<int,int>> mani1(state.manipulators.begin(), state.manipulators.end());
		mani1.insert({0,0});
		int dir = state.cur_dir;
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
				if (is_valid(state, cmds)) {
					valid_cmds.push_back(cmds);
					if (depth > 1) walk(depth-1);
				}
				cmds.pop_back();
			}
		}
	}

	int calc_score1(const vector<Command>& cc) const {
		int cur_y = state.cur_y;
		int cur_x = state.cur_x;
		int cur_dir = state.cur_dir;
		int cur_fast = state.cur_fast;
		int cur_drill = state.cur_drill;
		auto cur_boosters = state.cur_boosters;
		set<pair<int,int>> diff_rotten;
		set<pair<int,int>> drilled;
		int num_mani = 0;
		for (const auto& c : cc) {
			if (c.c == 'B') ++ num_mani;
			for (int t = 0; t < (c.twice ? 2 : 1); ++ t) {
				move2(cur_y, cur_x, cur_dir, cur_fast, cur_drill, cur_boosters, c.c);

				if (state.rotten[cur_y][cur_x]) diff_rotten.insert({cur_y, cur_x});
				for (const auto& v : state.manipulators) {
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
		return (int)diff_rotten.size() + num_mani * 10000;
	}

	int calc_score2(const Command& c) const {
		int cur_y = state.cur_y;
		int cur_x = state.cur_x;
		int cur_dir = state.cur_dir;
		int cur_fast = state.cur_fast;
		int cur_drill = state.cur_drill;
		auto cur_boosters = state.cur_boosters;
		set<pair<int,int>> diff_rotten;
		for (int t = 0; t < (c.twice ? 2 : 1); ++ t) {
			move2(cur_y, cur_x, cur_dir, cur_fast, cur_drill, cur_boosters, c.c);

			if (state.rotten[cur_y][cur_x]) diff_rotten.insert({cur_y, cur_x});
			for (const auto& v : state.manipulators) {
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
		if (fst1.empty()) throw 1;
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
		if (fst2.empty()) throw 1;
		return fst2[rand() % fst2.size()];
	}
};

void solve(const M& m) {
	State inst(m);

	for (;;) {
		if (inst.check_finish()) break;
		Solver solver(inst, 3);
		inst.input(solver.choose());
	}

	cout << inst.result << endl;
	//cerr << inst.turn << endl;
}
