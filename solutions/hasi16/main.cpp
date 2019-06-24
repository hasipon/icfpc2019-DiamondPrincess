#include "solve.hpp"

int main(int argc, char *argv[])
{
	srand(time(0));
	assert((argc == 3 || argc == 4)  && "invalid arguments");
	const string filename = argv[2];

	M m(filename);

	string buy;
	if (argc == 4) {
		if (argv[3][0] == '_') {
			buy = string(&argv[3][1]);
		} else {
			ifstream fin(argv[3]);
			fin >> buy;
		}
	}

	solve(m, buy, argv[2], argv[1]);
}
