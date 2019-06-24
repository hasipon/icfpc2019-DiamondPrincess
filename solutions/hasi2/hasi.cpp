#include "solve.hpp"

using namespace std;

const char* CMD = "WASDEQ";

void move(int& y, int& x, int& dir, char c) {
	switch (c) {
	case 'W': -- y; break;
	case 'A': -- x; break;
	case 'S': ++ y; break;
	case 'D': ++ x; break;
	case 'E': dir = (dir + 1) % 4; break;
	case 'Q': dir = (dir + 3) % 4; break;
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

struct Command {
	char c;
	Command(char c) : c(c) {}
};

bool operator<(const Command &lhs, const Command &rhs) {
    return lhs.c < rhs.c;
}

struct State {
	int H, W;
	vector<vector<bool>> board_init; // true: 進入可能
	map<pair<int,int>, vector<char>> boosters;

	string result;
	int cur_y, cur_x, cur_dir;
	vector<vector<bool>> visited;
	vector<char> cur_boosters;
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
		manipulators = {{-1,+1}, { 0,+1}, {-1,+1}};
		turn = 0;

		wrap();
	}

	void wrap() {
		rotten[cur_y][cur_x] = false;
		for (const auto& v : manipulators) {
			auto vv = rotate(v, cur_dir);
			int yy = cur_y + vv.first;
			int xx = cur_x + vv.second;
			// todo 到達可能判定
			if (0 <= yy && yy < H && 0 <= xx && xx < W) rotten[yy][xx] = false;
		}
		if (!visited[cur_y][cur_x]) {
			visited[cur_y][cur_x] = true;
			if (boosters.count({cur_y, cur_x})) {
				const auto& a = boosters[{cur_y, cur_x}];
				cur_boosters.insert(cur_boosters.end(), a.begin(), a.end());
			}
		}
	}

	bool check_finish() const {
		for (int i = 0; i < H; ++ i) for (int j = 0; j < W; ++ j) if (rotten[i][j]) return false;
		return true;
	}

	void input(const Command cc) {
		// cerr << cur_y << " " << cur_x << " " << cur_dir << " " << cc.c << endl;
		result += cc.c;
		turn += 1;
		move(cur_y, cur_x, cur_dir, cc.c);
		wrap();
	}
};

bool is_valid(const State& state, const vector<Command>& cmds) {
	//vector<vector<bool>> visited;
	//vector<char> cur_boosters;
	//vector<vector<bool>> rotten;
	//vector<pair<int,int>> manipulators;
	int cur_y = state.cur_y;
	int cur_x = state.cur_x;
	int cur_dir = state.cur_dir;
	for (const auto& c : cmds) {
		move(cur_y, cur_x, cur_dir, c.c);
		if (!(0 <= cur_y && cur_y < state.H && 0 <= cur_x && cur_x < state.W)) return false;
		if (!state.board_init[cur_y][cur_x]) return false;
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
		for (int i = 0; i < 6; ++ i) {
			auto c = Command(CMD[i]);
			cmds.push_back(c);
			if (is_valid(state, cmds)) {
				valid_cmds.push_back(cmds);
				if (depth > 1) walk(depth-1);
			}
			cmds.pop_back();
		}
	}

	int calc_score1(const vector<Command>& cc) const {
		int cur_y = state.cur_y;
		int cur_x = state.cur_x;
		int cur_dir = state.cur_dir;
		set<pair<int,int>> diff_rotten;
		for (const auto& c : cc) {
			move(cur_y, cur_x, cur_dir, c.c);

			if (state.rotten[cur_y][cur_x]) diff_rotten.insert({cur_y, cur_x});
			for (const auto& v : state.manipulators) {
				auto vv = rotate(v, cur_dir);
				int yy = cur_y + vv.first;
				int xx = cur_x + vv.second;
				// todo 到達可能判定
				if (0 <= yy && yy < state.H && 0 <= xx && xx < state.W && state.rotten[yy][xx]) {
					diff_rotten.insert({yy, xx});
				}
			}
		}
		return diff_rotten.size();
	}

	int calc_score2(const Command& c) const {
		int cur_y = state.cur_y;
		int cur_x = state.cur_x;
		int cur_dir = state.cur_dir;
		move(cur_y, cur_x, cur_dir, c.c);
		set<pair<int,int>> diff_rotten;
		if (state.rotten[cur_y][cur_x]) diff_rotten.insert({cur_y, cur_x});
		for (const auto& v : state.manipulators) {
			auto vv = rotate(v, cur_dir);
			int yy = cur_y + vv.first;
			int xx = cur_x + vv.second;
			// todo 到達可能判定
			if (0 <= yy && yy < state.H && 0 <= xx && xx < state.W && state.rotten[yy][xx]) {
				diff_rotten.insert({yy, xx});
			}
		}

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
				if (0 <= yy && yy < state.H && 0 <= xx && xx < state.W && state.board_init[yy][xx]) {
					if (state.rotten[yy][xx] && !diff_rotten.count({yy,xx})) return -dist;

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
			if (s > max_score2) {
				max_score2 = s;
				fst2 = {c};
			} else if (s == max_score2) {
				fst2.push_back(c);
			}
		}
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
	cerr << inst.turn << endl;
}
