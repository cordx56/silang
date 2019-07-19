#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>
#include <string>
#include <sstream>
#include <fstream>

class Interpreter;
struct Identifier;

typedef void (*funcPtr)(Interpreter&, std::vector<Identifier>);

struct VarVal {
	int v_int;
	double v_double;
	std::string v_string;
	funcPtr callPtr;
};

struct Identifier {
	std::string ident;
	std::string identType;
	std::string type;
	int scope;
	bool infix;
	VarVal v;
};

struct VarTable {
	std::unordered_map<std::string, double> doubleTable;
	std::unordered_map<std::string, int> intTable;
	std::unordered_map<std::string, std::string> stringTable;
};

class Interpreter {
private:
	std::vector<std::vector<std::string>> source;
	std::unordered_map<std::string, std::string> decldType = {
	};
	std::vector<std::unordered_map<std::string, Identifier>> decldIdent = {{
		{ "decas", { "decas", "func", "void", 0, true, { {}, {}, {}, NULL } }},
		{ "::", { "::", "func", "void", 0, true, { {}, {}, {}, NULL } }},
		{ "=", { "=", "func", "void", 0, true, { {}, {}, {}, NULL } }},
		{ "+", { "+", "func", "double", 0, true, { {}, {}, {}, NULL } }},
		{ "-", { "-", "func", "double", 0, true, { {}, {}, {}, NULL } }},
		{ "*", { "*", "func", "double", 0, true, { {}, {}, {}, NULL } }},
		{ "/", { "/", "func", "double", 0, true, { {}, {}, {}, NULL } }},
		{ "+=", { "+=", "func", "void", 0, true, { {}, {}, {}, NULL } }},
		{ "-=", { "-=", "func", "void", 0, true, { {}, {}, {}, NULL } }},
		{ "*=", { "*=", "func", "void", 0, true, { {}, {}, {}, NULL } }},
		{ "/=", { "/=", "func", "void", 0, true, { {}, {}, {}, NULL } }},
		{ "==", { "==", "func", "void", 0, true, { {}, {}, {}, NULL } }},
		{ "<", { "<", "func", "bool", 0, true, { {}, {}, {}, NULL } }},
		{ ">", { ">", "func", "bool", 0, true, { {}, {}, {}, NULL } }},
		{ "<=", { "<=", "func", "bool", 0, true, { {}, {}, {}, NULL } }},
		{ ">=", { ">=", "func", "bool", 0, true, { {}, {}, {}, NULL } }},
		{ "!", { "!", "func", "bool", 0, false, { {}, {}, {}, NULL } }},
		{ "if", { "if", "func", "void", 0, false, { {}, {}, {}, NULL } }},
		{ "loop", { "loop", "func", "void", 0, false, { {}, {}, {}, NULL } }},
		{ "{", { "{", "func", "void", 0, false, { {}, {}, {}, NULL } }},
		{ "}", { "}", "func", "void", 0, false, { {}, {}, {}, NULL } }},
		{ "println", { "println", "func", "void", 0, false, { {}, {}, {}, NULL } }}
	}};
	std::unordered_map<int, int> loopScope;
public:
	bool debug = false;
	int readLine;
	int readCol;
	int readScope;
	std::vector<Identifier> instStack;

	void parse(std::string);
	void readFile(std::string);
	std::string readNext();
	Identifier identOf(std::string);
	Identifier castTo(Identifier, std::string);
	Identifier identArithmetic(std::string, Identifier, Identifier);
	Identifier identCmp(std::string, Identifier, Identifier);
	void jumpBlock(int);
	void callFunc(std::vector<Identifier>);
	int run();
	void printSource();

	void declareIdent(std::string, Identifier);
};
