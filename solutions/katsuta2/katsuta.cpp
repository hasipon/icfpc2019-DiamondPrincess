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
const string COMMAND = "DSAWQE";

int H,W;
const int INF = 1e8;

vector<vector<bool> > obstacles;

int calcScore(vector<vector<bool> >& before, vector<vector<bool> >& after, P p) {
  int ret = 0;
  for(int i = -1 ; i <= 1 ; i++){
    for(int j = -1 ; j <= 1 ; j++){
      P np = P(p.first + i, p.second + j);
      if(!(0 <= np.first && np.first < H && 0 <= np.second && np.second < W))continue;
      if(before[np.first][np.second] && !after[np.first][np.second])ret++;
    }
  }
  return ret;
}

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

// bool moveCheck(vector<vector<bool> > flags, P p, int dir){
//   if(dir == 0){
//     if(p.second == W-1)return false;
//     if(p.first == 0)return false;
//     if(p.first == H-1)return false;
//   }

//   if(dir == 1){
//     if(p.second == 0)return false;
//     if(p.second == W-1) return false;
//     if(p.first == H-1)return false;
//   }

//   if(dir == 2){
//     if(p.second == 0)return false;
//     if(p.first == 0)return false;
//     if(p.first == H-1)return false;
//   }

//   if(dir == 3){
//     if(p.first == 0)return false;
//     if(p.second == 0)return false;
//     if(p.second == W-1)return false;
//   }
//   return true;
// }

bool isContainNotDrowArea(vector<vector<bool> >& b, P p ,int dir) {
  for (int i = 0; i < 3; ++ i) {
    int yy = p.first + dy[dir][i];
    int xx = p.second + dx[dir][i];
    if (0 <= yy && yy < b.size() && 0 <= xx && xx < b[yy].size()) {
      if(b[p.first][p.second])return true;
    }
  }
  return false;
}

int bfs(vector<vector<bool> >& flags, P sp, int dir){
  
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

      P np = P(p.first + _dy[i], p.second + _dx[i]);
      
      //if(!moveCheck(obstacles, np, dir))continue;
      if(can_walk(obstacles, np.first, np.second)){
        if(cost + 1 < d[np.first][np.second]){

          d[np.first][np.second] = cost + 1;
          path_y[np.first][np.second] = _dy[(i + 2)%4];
          path_x[np.first][np.second] = _dx[(i + 2)%4];
          direction[np.first][np.second] = i;
          que.push(PP(cost+1, np));
          if(isContainNotDrowArea(flags, np ,dir)){
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
 
// P(score, command)

void display(vector<vector<bool> > flags){
  for(int i = 0 ; i < flags.size() ; i++){
    for(int j = 0 ; j < flags[i].size() ; j++){
      cerr << flags[i][j];
    }
    cerr << endl;
  }
}

P dfs(vector<vector<bool> > flags, P p, int dir, int lim){
  if(lim == 0)return P(0, -1);
  
  int max_score = -1;
  int command = -1; // right, down, left, up, clock-wise, clounter-clock-wise

  for(int i = 0 ; i < 4 ; i++){
    P np = P(p.first + _dy[i], p.second + _dx[i]);
    if(!(0 <= np.first && np.second < H && 0 <= np.second && np.second < W))continue;
    //if(!moveCheck(flags, np, dir))continue;
    if(!can_walk(obstacles, np.first, np.second))continue;
    vector<vector<bool> > flags2 = flags;
    wrap(flags2, np.first, np.second, dir);
    int score = calcScore(flags, flags2, np);
    P dfs_res = dfs(flags2, np, dir, lim-1);
    if(max_score < dfs_res.first + score){
      max_score = dfs_res.first + score;
      command = i;
    }
  }

  for(int i = -1 ;  i <= 1 ; i++){
    if(i == 0)continue;
    dir = (dir + i + 4) % 4;
    //if(!moveCheck(flags, p, dir))continue;
    P np = p;
    vector<vector<bool> > flags2 = flags;
    wrap(flags2, np.first, np.second, dir);
    int score = calcScore(flags, flags2, np);
    P dfs_res = dfs(flags2, np, dir, lim-1);
    if(max_score < dfs_res.first + score){
      max_score = dfs_res.first + score;
      command = 4 + (i == 1);
    }
  }
  
  return P(max_score, command);
}

void solve(const M& m) {
  vector<vector<bool> > _flags(m.grid.size(), vector<bool>(m.grid[0].size()));
  for (unsigned i = 0; i < m.grid.size(); ++ i) for (unsigned j = 0; j < m.grid[i].size(); ++ j) {
      _flags[i][j] = (m.grid[i][j] != 0);
    }
  obstacles = _flags;
  vector<vector<bool> > flags = _flags;

  int dir = 0;

  P p = m.ini;
  H = _flags.size();
  W = _flags[0].size();

  wrap(flags, p.first, p.second, 0);

  while(!check_finish(flags)){

    P dfs_res = dfs(flags, p, dir, 1);
    int command = dfs_res.second;

    if(!(dfs_res.first == 0 || command == -1)){
      if(dfs_res.second < 4) {
        p = P(p.first + _dy[dfs_res.second], p.second + _dx[dfs_res.second]);
      }else {
        if(dfs_res.second == 4) dir = (dir + 3)%4;
        if(dfs_res.second == 5) dir = (dir + 1)%4;
      }
    }
    else {
      command = bfs(flags, p, dir);
      p = P(p.first + _dy[command], p.second + _dx[command]);
    }
    wrap(flags, p.first, p.second, dir);
    cout << COMMAND[command];
  }
}
