#ifndef SOLVE_HPP
#define SOLVE_HPP

#include <algorithm>
#include <deque>
#include <fstream>
#include <iostream>
#include <istream>
#include <iterator>
#include <map>
#include <numeric>
#include <ostream>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <vector>

#include <cassert>

#define each(i, c) for (auto& i : c)
#define unless(cond) if (!(cond))

using namespace std;

struct B {
  int x;
  int y;
  char c;
};

struct M {
  vector<vector<int>> grid;
  pair<int, int> ini;
  vector<B> boosters;

  M (string filename)
  {
    ifstream fin(filename.c_str());
    string line;
    getline(fin, line);
    replace(line.begin(), line.end(), '#', ' ');

    istringstream iss(line);
    string m, p, o, s, b;
    iss >> m >> p >> o >> b;
    parseMap(m);
    parsePoint(p);
    parseObstacles(o);
    parseBoosters(b);

    vector<vector<int>> tmp = grid;
    for (int i = 0; i < tmp.size(); ++i) {
      for (int j = 0; j < tmp[i].size(); ++j) {
        grid[i][j] = tmp[tmp.size() - i - 1][j];
      }
    }
  }

  void _parseBoosters(string& b)
  {
    replace(b.begin(), b.end(), '(', ' ');
    replace(b.begin(), b.end(), ',', ' ');
    replace(b.begin(), b.end(), ')', ' ');
    istringstream iss(b);
    B booster;
    iss >> booster.c >> booster.x >> booster.y;
    booster.y = h() - booster.y - 1;
    boosters.push_back(booster);
    return ;
  }

  void parseBoosters(string& b)
  {
    replace(b.begin(), b.end(), ';', ' ');
    istringstream iss(b);
    for (string s; iss >> s; _parseBoosters(s)) ;
    return ;
  }

  void parseMap(string& m)
  {
    static const int di[] = {-1, 0, +1, 0};
    static const int dj[] = {0, +1, 0, -1};

    replace(m.begin(), m.end(), '(', ' ');
    replace(m.begin(), m.end(), ')', ' ');
    replace(m.begin(), m.end(), ',', ' ');
    istringstream iss(m);

    vector<pair<int ,int>> v;
    pair<int, int> p;
    while (iss >> p.second >> p.first) {
      v.push_back(p);
    }

    int mxh = 0;
    int mxw = 0;
    for (int i = 0; i < v.size(); ++i) {
      mxh = max(mxh, v[i].first);
      mxw = max(mxw, v[i].second);
    }

    grid.resize(mxh);
    for (int i = 0; i < grid.size(); ++i) {
      grid[i].resize(mxw, 0);
    }

    vector<pair<int, int>> u;
    for (int i = 0; i < v.size(); ++i) {
      const int j = (i + 1) % v.size();
      if (v[i].first < v[j].first) {
        u.push_back(v[i]);
        u.push_back(v[j]);
      }
      if (v[i].first > v[j].first) {
        u.push_back(v[j]);
        u.push_back(v[i]);
      }
    }

    for (int i = 0; i < u.size(); i += 2) {
      const int j = i + 1;
      int d = -1;
      if (u[i].first < u[j].first) d = 2;
      if (u[i].first > u[j].first) d = 0;
      if (d == -1) continue;
      assert(d != -1);
      pair<int, int> p = u[i];
      while (true) {
        auto q = p;
        for (int k = 0; isInside(q.first, q.second); ++k) {
          grid[q.first][q.second] ^= 1;
          q.first += di[1];
          q.second += dj[1];
        }
        p.first += di[d];
        p.second += dj[d];
        if (p == u[j]) break;
      }
    }

    return ;
  }

  void parsePoint(string& m)
  {
    replace(m.begin(), m.end(), '(', ' ');
    replace(m.begin(), m.end(), ')', ' ');
    replace(m.begin(), m.end(), ',', ' ');
    istringstream iss(m);
    iss >> ini.second >> ini.first;
    ini.first = h() - ini.first - 1;
    return ;
  }

  void _parseObstacles(string& m)
  {
    static const int di[] = {-1, 0, +1, 0};
    static const int dj[] = {0, +1, 0, -1};

    replace(m.begin(), m.end(), '(', ' ');
    replace(m.begin(), m.end(), ')', ' ');
    replace(m.begin(), m.end(), ',', ' ');
    istringstream iss(m);

    vector<pair<int ,int>> v;
    pair<int, int> p;
    while (iss >> p.second >> p.first) {
      v.push_back(p);
    }

    vector<pair<int, int>> u;
    for (int i = 0; i < v.size(); ++i) {
      const int j = (i + 1) % v.size();
      if (v[i].first < v[j].first) {
        u.push_back(v[i]);
        u.push_back(v[j]);
      }
      if (v[i].first > v[j].first) {
        u.push_back(v[j]);
        u.push_back(v[i]);
      }
    }

    for (int i = 0; i < u.size(); i += 2) {
      const int j = i + 1;
      pair<int, int> p = u[i];
      while (true) {
        auto q = p;
        for (int k = 0; isInside(q.first, q.second); ++k) {
          grid[q.first][q.second] ^= 1;
          q.first += di[1];
          q.second += dj[1];
        }
        p.first += di[2];
        p.second += dj[2];
        if (p == u[j]) break;
      }
    }
    return ;
  }

  void parseObstacles(string& m)
  {
    replace(m.begin(), m.end(), ';', ' ');
    istringstream iss(m);
    string s;
    while (iss >> s) {
      _parseObstacles(s);
    }
  }

  bool isInside(int i, int j) const
  {
    if (!(0 <= i && i < grid.size())) return false;
    if (!(0 <= j && j < grid[0].size())) return false;
    return true;
  }

  bool isBlocked(int i, int j) const
  {
    return isInside(i, j) && !grid[i][j];
  }

  bool blocked(int i, int j) const
  {
    return isBlocked(i, j);
  }

  bool inside(int i, int j) const
  {
    return isInside(i, j);
  }

  size_t h(void) const
  {
    return grid.size();
  }

  size_t w(void) const
  {
    return grid[0].size();
  }

  void show(void)
  {
    for (int i = 0; i < h(); ++i) {
      for (int j = 0; j < w(); ++j) {
        cout << (int)grid[i][j];
      }
      cout << endl;
    }
    return ;
  }
};

void solve(const M& m);

#endif /* SOLVES_HPP */
