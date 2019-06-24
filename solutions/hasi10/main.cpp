#include "solve.hpp"

int main(int argc, char *argv[])
{
	assert((argc == 2 || argc == 3)  && "invalid arguments");
	const string filename = argv[1];

	M m(filename);

	string buy;
	if (argc == 3) {
		if (argv[2][0] == '_') {
			buy = string(&argv[2][1]);
		} else {
			ifstream fin(argv[2]);
			fin >> buy;
		}
	}

	solve(m, buy, argv[1]);
}
