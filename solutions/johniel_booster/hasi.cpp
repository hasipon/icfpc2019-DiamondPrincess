#include "solve.hpp"

using namespace std;

const char* CMD = "WASDEQ";

void wrap(vector<vector<bool>>& b, int y, int x, int dir) {
	b[y][x] = false;
	int dy[4][3] = {{-1, 0,+1}, {+1,+1,+1}, {-1, 0,+1}, {-1,-1,-1}};
	int dx[4][3] = {{+1,+1,+1}, {-1, 0,+1}, {-1,-1,-1}, {-1, 0,+1}};
	for (int i = 0; i < 3; ++ i) {
		int yy = y + dy[dir][i];
		int xx = x + dx[dir][i];
		if (0 <= yy && yy < b.size() && 0 <= xx && xx < b[yy].size()) b[yy][xx] = false;
	}
}

bool check_finish(const vector<vector<bool>>& b) {
	for (auto& bb : b) for (bool x : bb) if (x) return false;
	return true;
}

typedef pair<int,int> score_t;
score_t SCORE_MIN = {-1,0};

score_t calc_score(const vector<vector<bool>>& a, const vector<vector<bool>>& b, int y, int x) {
	score_t score = {0, 0};
	for (auto& bb : b) for (bool x : bb) if (!x) ++ score.first;
	auto aa = a;
	queue<pair<pair<int,int>,int>> q;
	q.push({{y,x},0});
	aa[y][x] = false;
	int dy[] = {-1, 0,+1, 0};
	int dx[] = { 0,-1, 0,+1};
	while (!q.empty()) {
		int yy = q.front().first.first;
		int xx = q.front().first.second;
		int dist = q.front().second;
		q.pop();
		if (b[yy][xx]) {
			score.second = -dist;
			break;
		}
		for (int i = 0; i < 4; ++ i) {
			int yyy = yy + dy[i];
			int xxx = xx + dx[i];
			if (0 <= yyy && yyy < aa.size() && 0 <= xxx && xxx < aa[yyy].size() && aa[yyy][xxx]) {
				q.push({{yyy,xxx},dist+1});
				aa[yyy][xxx] = false;
			}
		}
	}
	return score;
}

bool can_walk(const vector<vector<bool>>& a, int y, int x) {
	return 0 <= y && y < a.size() && 0 <= x && x < a[y].size() && a[y][x];
}

void move(int& y, int& x, int& dir, char c) {
	switch (c) {
	case 'W': -- y; break;
	case 'A': -- x; break;
	case 'S': ++ y; break;
	case 'D': ++ x; break;
	case 'E': dir = (dir + 1) % 4; break;
	case 'Q': dir = (dir + 3) % 4; break;
	}
}

score_t walk(const vector<vector<bool>>& a, const vector<vector<bool>>& b, int y, int x, int dir, char c, int depth) {
	int y2 = y, x2 = x, dir2 = dir;
	move(y2, x2, dir2, c);
	if (!can_walk(a, y2, x2)) return SCORE_MIN;
	auto b2 = b;
	wrap(b2, y2, x2, dir2);
	if (depth == 0) return calc_score(a, b2, y2, x2);
	auto s = SCORE_MIN;
	for (int i = 0; i < 6; ++ i) {
		s = max(s, walk(a, b2, y2, x2, dir2, CMD[i], depth-1));
	}
	return s;
}

void solve(const M& m) {
	vector<vector<bool>> a(m.grid.size(), vector<bool>(m.grid[0].size()));
	for (unsigned i = 0; i < m.grid.size(); ++ i) for (unsigned j = 0; j < m.grid[i].size(); ++ j) {
		a[i][j] = (m.grid[i][j] != 0);
	}

	vector<vector<bool>> b = a;

	int y = m.ini.first, x = m.ini.second, dir = 0;
	for (;;) {
		wrap(b, y, x, dir);
		if (check_finish(b)) break;
		vector<char> ch;
		for (int i = 0; i < 6; ++ i) ch.push_back(CMD[i]);
		for (int t = 2; t >= 0 && ch.size() >= 2; -- t) {
			auto score = SCORE_MIN;
			vector<char> ch2;
			for (auto c : ch) {
				auto s = walk(a, b, y, x, dir, c, t);
				if (s > score) {
					ch2 = {c};
					score = s;
				} else if (s == score) {
					if (s == SCORE_MIN) continue;
					ch2.push_back(c);
				}
			}
			ch = ch2;
		}
		auto sel = ch[rand() % ch.size()];
		cout << sel;
		move(y, x, dir, sel);
	}
	cout << endl;
}
