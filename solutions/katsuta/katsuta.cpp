#include "solve.hpp"

using namespace std;

const char* CMD = "WASDEQ";

typedef pair<int,int>P;
typedef pair<int,P>PP;

const int _dy[] = {0, 1, 0, -1};
const int _dx[] = {1, 0, -1, 0};
//                                   Right        DOWN     LEFT        UP
const int dy[4][3] = {{-1, 0,+1}, {+1,+1,+1}, {-1, 0,+1}, {-1,-1,-1}};
const int dx[4][3] = {{+1,+1,+1}, {-1, 0,+1}, {-1,-1,-1}, {-1, 0,+1}};
const string COMMAND = "DSAW";

int H,W;
const int INF = 1e8;

vector<vector<bool> > obstacles;

void wrap(vector<vector<bool> >& b, int y, int x, int dir) {
	b[y][x] = false;
	for (int i = 0; i < 3; ++ i) {
		int yy = y + dy[dir][i];
		int xx = x + dx[dir][i];
		if (0 <= yy && yy < b.size() && 0 <= xx && xx < b[yy].size()) b[yy][xx] = false;
	}
}

bool check_finish(const vector<vector<bool> >& b) {
	for (auto& bb : b) for (bool x : bb) if (x) return false;
	return true;
}
bool can_walk(const vector<vector<bool> >& a, int y, int x) {
  return 0 <= y && y < a.size() && 0 <= x && x < a[y].size() && a[y][x];
}

// void move(int& y, int& x, int& dir, char c) {
// 	switch (c) {
// 	case 'W': -- y; break;
// 	case 'A': -- x; break;
// 	case 'S': ++ y; break;
// 	case 'D': ++ x; break;
// 	case 'E': dir = (dir + 1) % 4; break;
// 	case 'Q': dir = (dir + 3) % 4; break;
// 	}
// }

int bfs(vector<vector<bool> >& flags, P sp){
  
  queue<PP>que;
  que.push(PP(0, sp));

  int path_y[H][W];
  int path_x[H][W];
  int direction[H][W];
  int d[H][W];

  for(int i= 0 ; i < H ; i++){
    for(int j = 0 ; j < W ; j++){
      path_y[i][j] = path_x[i][j] = INF;
      d[i][j] = INF;
      direction[i][j] = -1;
    }
  }

  while(que.size()){
    PP pp = que.front(); que.pop();
    int cost = pp.first;
    P p = pp.second;

    for(int i = 0 ; i < 4 ; i++){
      //cout << "i = " << i << endl;
      
      P np = P(p.first + _dy[i], p.second + _dx[i]);
      if(can_walk(obstacles, np.first, np.second)){
        // cout << "here" << endl;
        // cout << "cost = " << cost << endl;
        // cout << "d[np.first][np.second] = " << d[np.first][np.second] << endl;
          
        if(cost + 1 < d[np.first][np.second]){
          
          d[np.first][np.second] = cost + 1;
          path_y[np.first][np.second] = _dy[(i + 2)%4];
          path_x[np.first][np.second] = _dx[(i + 2)%4];
          direction[np.first][np.second] = i;
          que.push(PP(cost+1, np));
          if(flags[np.first][np.second]){
            vector<P>vec;
            P _p = np;
            while(1){
              P _np = P(_p.first + path_y[_p.first][_p.second], _p.second + path_x[_p.first][_p.second]);

              if(_np == sp){
                return direction[_p.first][_p.second];
              }
              _p = _np;
            }
          }
        }
      }
    }
  }
  return -1;
}

void solve(const M& m) {
  vector<vector<bool> > _flags(m.grid.size(), vector<bool>(m.grid[0].size()));
  for (unsigned i = 0; i < m.grid.size(); ++ i) for (unsigned j = 0; j < m.grid[i].size(); ++ j) {
      _flags[i][j] = (m.grid[i][j] != 0);
    }
  obstacles = _flags;
  vector<vector<bool> > flags = _flags;

  P p = m.ini;
  H = _flags.size();
  W = _flags[0].size();

  while(!check_finish(flags)){
    int direction = bfs(flags, p);
    p = P(p.first + _dy[direction], p.second + _dx[direction]);
    flags[p.first][p.second] = false;
    wrap(flags, p.first, p.second, 0);
    cout << COMMAND[direction];
  }
}
