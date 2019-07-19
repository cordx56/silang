#include "interpreter.hpp"

void Interpreter::parse(std::string source) {
	std::stringstream ss1(source);
	std::string buf1, buf2;
	while (std::getline(ss1, buf1)) {
		std::stringstream ss2(buf1);
		std::vector<std::string> line;
		while (std::getline(ss2, buf2, ' ')) {
			line.push_back(buf2);
		}
		this->source.push_back(line);
	}
	this->readLine = 0;
	this->readCol = 0;
	this->readScope = 0;
}

void Interpreter::readFile(std::string fn) {
	std::ifstream sourcefile(fn);
	std::stringstream ss;
	ss << sourcefile.rdbuf();
	sourcefile.close();
	std::string source(ss.str());
	this->parse(source);
}

std::string Interpreter::readNext() {
	std::string stringBuf;
	while (this->readLine < (int)this->source.size()) {
		while (this->readCol < (int)this->source[this->readLine].size()) {
			this->readCol++;
			if (!stringBuf.empty()) {
				if (this->source[this->readLine][this->readCol - 1].back() == '"') {
					stringBuf += " " + this->source[this->readLine][this->readCol - 1];
					return stringBuf;
				}
				stringBuf += " " + this->source[this->readLine][this->readCol - 1];
			} else if (this->source[this->readLine][this->readCol - 1].front() == '"') {
				stringBuf = this->source[this->readLine][this->readCol - 1];
				if (1 < stringBuf.size() &&stringBuf.back() == '"') return stringBuf;
				continue;
			} else if (!this->source[this->readLine][this->readCol - 1].empty()) {
				return this->source[this->readLine][this->readCol - 1];
			}
		}
		this->readCol = 0;
		this->readLine++;
		return "\n";
	}
	return "";
}

Identifier Interpreter::identOf(std::string ident) {
	for (int i = this->readScope; i >= 0; i--) {
		if (this->decldIdent[i].find(ident) != this->decldIdent[i].end()) return this->decldIdent[i][ident];
	}
	return { ident, "undefined", "undefined", 0, false, { {}, {}, {}, NULL } };
}

Identifier Interpreter::castTo(Identifier ident, std::string to) {
	if (ident.type == "int") {
		if (to == "double") ident.v.v_double = (double)ident.v.v_int;
		else if (to == "string") ident.v.v_string = std::to_string(ident.v.v_int);
	} else if (ident.type == "double") {
		if (to == "int") ident.v.v_int = (int)ident.v.v_double;
		else if (to == "string") ident.v.v_string = std::to_string(ident.v.v_double);
	} else if (ident.type == "bool") {
		if (to == "double") ident.v.v_double = (bool)ident.v.v_int;
		else if (to == "string") ident.v.v_string = (ident.v.v_int) ? "true" : "false";
	} else if (ident.type == "string") {
		if (to == "int") ident.v.v_int = std::stoi(ident.v.v_string);
		else if (to == "double") ident.v.v_double = std::stod(ident.v.v_string);
	} else if (ident.type == "undefined") {
		if (to == "int") ident.v.v_int = std::stoi(ident.ident);
		else if (to == "double") ident.v.v_double = std::stod(ident.ident);
		else if (to == "string") {
			if (ident.ident.front() == '"') ident.ident.erase(ident.ident.begin());
			if (ident.ident.back() == '"') ident.ident.pop_back();
			ident.v.v_string = ident.ident;
		}
	}
	ident.type = to;
	return ident;
}

Identifier Interpreter::identArithmetic(std::string opr, Identifier idefA, Identifier idefB) {
	Identifier ret = { "", "undefined", "undefined", 0, false, { {}, {}, {}, NULL} };
	if (idefA.type == idefB.type && idefA.type == "undefined") {
	}
	if (idefA.type == "string" || idefB.type == "string") {
		ret.type = "string";
		if (opr == "+") {
			ret.v.v_string = this->castTo(idefA, "string").v.v_string + this->castTo(idefB, "string").v.v_string;
		}
	} else if (idefA.type == "double" || idefB.type == "double") {
		ret.type = "double";
		if (opr == "+") {
			ret.v.v_double = this->castTo(idefA, "double").v.v_double + this->castTo(idefB, "double").v.v_double;
		} else if (opr == "-") {
			ret.v.v_double = this->castTo(idefA, "double").v.v_double - this->castTo(idefB, "double").v.v_double;
		} else if (opr == "*") {
			ret.v.v_double = this->castTo(idefA, "double").v.v_double * this->castTo(idefB, "double").v.v_double;
		} else if (opr == "/") {
			ret.v.v_double = this->castTo(idefA, "double").v.v_double / this->castTo(idefB, "double").v.v_double;
		}
	} else {
		ret.type = "int";
		if (opr == "+") {
			ret.v.v_int = this->castTo(idefA, "int").v.v_int + this->castTo(idefB, "int").v.v_int;
		} else if (opr == "-") {
			ret.v.v_int = this->castTo(idefA, "int").v.v_int - this->castTo(idefB, "int").v.v_int;
		} else if (opr == "*") {
			ret.v.v_int = this->castTo(idefA, "int").v.v_int * this->castTo(idefB, "int").v.v_int;
		} else if (opr == "/") {
			ret.v.v_int = this->castTo(idefA, "int").v.v_int / this->castTo(idefB, "int").v.v_int;
		}
	}
	return ret;
}

Identifier Interpreter::identCmp(std::string opr, Identifier idefA, Identifier idefB) {
	Identifier ret = { "", "undefined", "bool", 0, false, { {}, {}, {}, NULL} };
	if (idefA.type == idefB.type && idefA.type == "undefined") {
	}
	if (idefA.type == "string" || idefB.type == "string") {
		if (opr == "==") {
			ret.v.v_int = (this->castTo(idefA, "string").v.v_string == this->castTo(idefB, "string").v.v_string);
		}
	} else if (idefA.type == "double" || idefB.type == "double") {
		if (opr == "==") {
			ret.v.v_int = (this->castTo(idefA, "double").v.v_double == this->castTo(idefB, "double").v.v_double);
		} else if (opr == "<") {
			ret.v.v_int = (this->castTo(idefA, "double").v.v_double < this->castTo(idefB, "double").v.v_double);
		} else if (opr == ">") {
			ret.v.v_int = (this->castTo(idefA, "double").v.v_double > this->castTo(idefB, "double").v.v_double);
		} else if (opr == "<=") {
			ret.v.v_int = (this->castTo(idefA, "double").v.v_double <= this->castTo(idefB, "double").v.v_double);
		} else if (opr == ">=") {
			ret.v.v_int = (this->castTo(idefA, "double").v.v_double >= this->castTo(idefB, "double").v.v_double);
		}
	} else {
		if (opr == "==") {
			ret.v.v_int = (this->castTo(idefA, "int").v.v_int == this->castTo(idefB, "int").v.v_int);
		} else if (opr == "<") {
			ret.v.v_int = (this->castTo(idefA, "int").v.v_int < this->castTo(idefB, "int").v.v_int);
		} else if (opr == ">") {
			ret.v.v_int = (this->castTo(idefA, "int").v.v_int > this->castTo(idefB, "int").v.v_int);
		} else if (opr == "<=") {
			ret.v.v_int = (this->castTo(idefA, "int").v.v_int <= this->castTo(idefB, "int").v.v_int);
		} else if (opr == ">=") {
			ret.v.v_int = (this->castTo(idefA, "int").v.v_int >= this->castTo(idefB, "int").v.v_int);
		}
	}
	return ret;
}

void Interpreter::jumpBlock(int step = 0) {
	std::string buf;
	int target = this->readScope - step;
	while (!(buf = this->readNext()).empty()) {
		Identifier tmp = this->identOf(buf);
		if (tmp.identType == "func" && tmp.ident == "{") {
			this->readScope++;
		} else if (tmp.identType == "func" && tmp.ident == "}") {
			this->readScope--;
			if (this->readScope < target) {
				return;
			}
		}
	}
}

void Interpreter::callFunc(std::vector<Identifier> args) {
	if (args[0].ident == "decas" || args[0].ident == "::") {
		this->declareIdent(args[1].ident, 
				{ args[1].ident, "variable", args[2].ident, this->readScope, false, { {}, {}, {}, NULL }});
	} else if (args[0].ident == "=") {
		this->decldIdent[args[1].scope][args[1].ident].v = this->castTo(args[2], args[1].type).v;
	} else if (args[0].ident == "+" || args[0].ident == "-" || args[0].ident == "*" || args[0].ident == "/") {
		this->instStack.push_back(this->identArithmetic(args[0].ident, args[1], args[2]));
	} else if (args[0].ident == "+=" || args[0].ident == "-=" || args[0].ident == "*=" || args[0].ident == "/=") {
		std::string tmpIdent = args[0].ident;
		tmpIdent.pop_back();
		this->decldIdent[args[1].scope][args[1].ident].v =
				this->identArithmetic(tmpIdent, args[1], this->castTo(args[2], args[1].type)).v;
	} else if (args[0].ident == "==" || args[0].ident == "<" || args[0].ident == ">" || args[0].ident == "<=" || args[0].ident == ">=") {
		this->instStack.push_back(this->identCmp(args[0].ident, args[1], args[2]));
	} else if (args[0].ident == "!") {
		Identifier tmp = { "", "undefined", "bool", 0, false, { {}, {}, {}, NULL }};
		tmp.v.v_int = !this->castTo(args[1], "bool").v.v_int;
		this->instStack.push_back(tmp);
	} else if (args[0].ident == "if") {
		if (1 < args.size() && args[1].v.v_int) {
		} else {
			this->jumpBlock();
			this->decldIdent.pop_back();
		}
	} else if (args[0].ident == "loop") {
		if (1 < args.size() && args[1].v.v_int) {
			this->loopScope[this->readScope] = this->readLine - 1;
		} else {
			this->jumpBlock();
			this->decldIdent.pop_back();
		}
	} else if (args[0].ident == "{") {
		this->readScope++;
		if ((int)this->decldIdent.size() < this->readScope + 1) 
			this->decldIdent.push_back({});
	} else if (args[0].ident == "}") {
		if (this->loopScope.find(this->readScope) != this->loopScope.end()) {
			this->readLine = this->loopScope[this->readScope];
			this->readCol = 0;
			this->loopScope.erase(this->readScope);
		} else {
			this->decldIdent.pop_back();
		}
		this->readScope--;
	} else if (args[0].ident == "println") {
		for (unsigned long i = 1; i < args.size(); i++) {
			std::cout << this->castTo(args[i], "string").v.v_string;
		}
		std::cout << std::endl;
	} else if (args[0].v.callPtr != NULL) {
		args[0].v.callPtr(*this, args);
	}
}

int Interpreter::run() {
	std::string buf;
	while (!(buf = this->readNext()).empty()) {
		if (buf == "\n") {
			std::vector<Identifier> args;
			Identifier tmp;
			while (0 < this->instStack.size()) {
				tmp = this->instStack.back();
				this->instStack.pop_back();
				if (tmp.identType == "func") {
					if (tmp.infix && args.size() == 1 && 0 < this->instStack.size()) {
						args.push_back(this->instStack.back());
						this->instStack.pop_back();
					}
					args.push_back(tmp);
					std::reverse(args.begin(), args.end());
					this->callFunc(args);
					args.clear();
				} else {
					args.push_back(tmp);
				}
			}
		} else {
			this->instStack.push_back(this->identOf(buf));
			buf += " :: " + this->instStack.back().identType + "->" + this->instStack.back().type;
			if (this->debug) std::cout << buf << std::endl;
		}
	}
	return 0;
}

void Interpreter::printSource() {
}

void Interpreter::declareIdent(std::string ident, Identifier data) {
	if (this->decldIdent[data.scope].find(ident) != this->decldIdent[data.scope].end()) {
		return;
	}
	this->decldIdent[data.scope][ident] = data;
}
