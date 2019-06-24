#include "solve.hpp"

int main(int argc, char *argv[])
{
	assert(argc == 2 && "invalid arguments");
	const string filename = argv[1];

	M m(filename);

	solve(m);
}
