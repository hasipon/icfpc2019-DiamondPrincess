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

int main() {
	for (;;) {
		int y, x;
		cin >> y >> x;
		for (auto p : calc_obstacles({y,x})) {
			cout << p.first << " " << p.second << endl;
		}
	}
}
