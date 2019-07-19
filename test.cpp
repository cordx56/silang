#include "interpreter.hpp"
#include <iostream>

int main(int argc, char* argv[]) {
	Interpreter rs;
	if (2 < argc) rs.debug = true;

	rs.readFile(argv[1]);
	rs.run();
	return 0;
}
