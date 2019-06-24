#include <iostream>

#define each(i, c) for (auto& i : c)
#define unless(cond) if (!(cond))

using namespace std;

int main(int argc, char *argv[])
{
    for(int i = 0; i<argc; i++){
        cout << string(argv[i]) <<endl;
    }
  return 0;
}
