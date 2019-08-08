#include "silinterpreter.hpp"
#include <iostream>

int main(int argc, char* argv[]) {
	std::string filename;
	std::string inlinePg;
	bool htmlOutput = false;
	bool outputParseTree = false;
	for (int i = 1; i < argc; i++) {
		if (argv[i][0] == '-') {
			std::string opt = argv[i];
			if (opt == "--html") htmlOutput = true;
			if (opt == "--parseTree") outputParseTree = true;
			if (opt == "-i" && i + 1 < argc) inlinePg = argv[++i];
		} else {
			filename = argv[i];
		}
	}
	try {
		sil::Interpreter rs;
		if (filename.empty() && inlinePg.empty()) {
			std::string inst;
			while (1) {
				std::cout << "> " << std::flush;
				std::getline(std::cin, inst);
				if (inst == "quit") break;
				try {
					sil::Statement stmt = rs.parse(inst);
					if (stmt.isBlock() && stmt.getStatements().size() == 1) {
						stmt = stmt.getStatements()[0];
					}
					rs.run(stmt);
				} catch (sil::InterpreterException e) {
					std::cerr << e.what() << std::endl;
				}
			}
		} else {
			sil::Statement stmts;
			if (filename.empty()) stmts = rs.parse(inlinePg);
			else stmts = rs.parseFile(filename);

			if (outputParseTree) {
				if (htmlOutput) std::cout << "<pre class=\"silParseTree\"><code>";
				std::cout << stmts.statementTree();
				if (htmlOutput) std::cout << "</code></pre>";
				std::cout << std::endl;
			}
			if (htmlOutput) std::cout << "<pre class=\"silOutput\"><code>";
			rs.run(stmts);
			if (htmlOutput) std::cout << "</code></pre>";
		}
	} catch (sil::InterpreterException e) {
		std::cerr << e.what() << std::endl;
	} catch (...) {
		std::cerr << "Unknown error!" << std::endl;
	}

	return 0;
}
