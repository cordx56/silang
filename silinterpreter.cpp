#include "silinterpreter.hpp"

// Identifier class
sil::Identifier::Identifier() {
	def = Definition::undefined;
	type = "undefined";
	scope = 0;
	infix = false;
	v.v_int = 0;
	v.v_double = 0.0;
	v.callPtr = nullptr;
}
sil::Identifier::Identifier(std::string str) : sil::Identifier::Identifier() {
	try {
		double val = std::stod(str);
		def = Definition::constant;
		type = "double";
		v.v_double = val;
		if ((double)val == (int)val) {
			type = "int";
			v.v_int = (int)val;
		}
	} catch (...) {
		surface = str;
	}
}
sil::Identifier& sil::Identifier::setSurface(std::string v) { surface = v; return *this; }
sil::Identifier& sil::Identifier::setDef(Definition v) { def = v; return *this; }
sil::Identifier& sil::Identifier::setType(std::string v) { type = v; return *this; }
sil::Identifier& sil::Identifier::setType(Identifier v) {
	if (!v.isType()) throw InterpreterException("");
	return setType(v.getSurface());
}
sil::Identifier& sil::Identifier::setScope(int v) { scope = v; return *this; }
sil::Identifier& sil::Identifier::setInfix(bool v) { infix = v; return *this; }
std::string sil::Identifier::getSurface() { return surface; }
sil::Identifier::Definition sil::Identifier::getDef() { return def; }
std::string sil::Identifier::getType() { return type; }
int sil::Identifier::getScope() { return scope; }
bool sil::Identifier::isInfix() { return infix; }
bool sil::Identifier::isFunction() { return (def == Definition::function); }
bool sil::Identifier::isVariable() { return (def == Definition::variable); }
bool sil::Identifier::isConstant() { return (def == Definition::constant); }
bool sil::Identifier::isType() { return (def == Definition::typeName); }
bool sil::Identifier::isUndefined() { return (def == Definition::undefined); }
bool sil::Identifier::isInvalid() { return (def == Definition::invalid); }
sil::Identifier& sil::Identifier::setValue(sil::Identifier::Value val) { v = val; return *this; }
sil::Identifier& sil::Identifier::setInt(int val) { v.v_int = val; return *this; }
sil::Identifier& sil::Identifier::setDouble(double val) { v.v_double = val; return *this; }
sil::Identifier& sil::Identifier::setBool(bool val) { v.v_int = val; return *this; }
sil::Identifier& sil::Identifier::setString(std::string val) { v.v_string = val; return *this; }
sil::Identifier& sil::Identifier::setFuncPtr(silFunc val) { v.callPtr = val; return *this; }
sil::Identifier::Value& sil::Identifier::getValue() { return v; }
int sil::Identifier::getInt() { return v.v_int; }
double sil::Identifier::getDouble() { return v.v_double; }
bool sil::Identifier::getBool() { return v.v_int; }
std::string sil::Identifier::getString() { return v.v_string; }
std::vector<sil::IdentRefId> sil::Identifier::callFunc(Interpreter& rs, std::vector<Expression> exprs) {
	if (v.callPtr != nullptr) return v.callPtr(rs, exprs);
	return rs.callBuiltinFunc(exprs);
}
std::vector<sil::IdentRefId>& sil::Identifier::getArray() { return v.array; }
sil::IdentRefId sil::Identifier::getArray(int i) { return v.array[i]; }
std::unordered_map<std::string, sil::IdentRefId>& sil::Identifier::getMap() { return v.map; }
sil::IdentRefId sil::Identifier::getMap(std::string i) { return v.map[i]; }
sil::Identifier sil::Identifier::castTo(std::string to) {
	Identifier cast;
	cast.setDef(Identifier::Definition::constant).setType(to);
	if (type == to) return *this;
	else if (type == "int") {
		if (to == "double") cast.v.v_double = (double)v.v_int;
		else if (to == "bool") cast.v.v_int = (bool)v.v_int;
		else if (to == "string") cast.v.v_string = std::to_string(v.v_int);
	} else if (type == "double") {
		if (to == "int") cast.v.v_int = (int)v.v_double;
		else if (to == "bool") cast.v.v_int = (bool)v.v_double;
		else if (to == "string") cast.v.v_string = std::to_string(v.v_double);
	} else if (type == "bool") {
		if (to == "double") cast.v.v_double = (double)v.v_int;
		else if (to == "string") cast.v.v_string = (v.v_int) ? "true" : "false";
	} else if (type == "string") {
		if (to == "int") cast.v.v_int = std::stoi(v.v_string);
		else if (to == "double") cast.v.v_double = std::stod(v.v_string);
		else if (to == "bool") cast.v.v_int = !v.v_string.empty();
	}
	return cast;
}
sil::Identifier sil::Identifier::operator +(Identifier& rv) {
	Identifier ret;
	ret.setDef(Definition::constant);
	if (type == "string" || rv.type == "string") {
		ret.setType("string");
		ret.setString(this->castTo("string").getString() + rv.castTo("string").getString());
	} else if (type == "double" || rv.type == "double") {
		ret.setType("double");
		ret.setDouble(this->castTo("double").getDouble() + rv.castTo("double").getDouble());
	} else if (type == "int" || rv.type == "int") {
		ret.setType("int");
		ret.setInt(this->castTo("int").getInt() + rv.castTo("int").getInt());
	}
	return ret;
}
sil::Identifier sil::Identifier::operator -(Identifier& rv) {
	Identifier ret;
	ret.setDef(Definition::constant);
	if (type == "double" || rv.type == "double") {
		ret.setType("double");
		ret.setDouble(this->castTo("double").getDouble() - rv.castTo("double").getDouble());
	} else if (type == "int" || rv.type == "int") {
		ret.setType("int");
		ret.setInt(this->castTo("int").getInt() - rv.castTo("int").getInt());
	}
	return ret;
}
sil::Identifier sil::Identifier::operator *(Identifier& rv) {
	Identifier ret;
	ret.setDef(Definition::constant);
	if (type == "double" || rv.type == "double") {
		ret.setType("double");
		ret.setDouble(this->castTo("double").getDouble() * rv.castTo("double").getDouble());
	} else if (type == "int" || rv.type == "int") {
		ret.setType("int");
		ret.setInt(this->castTo("int").getInt() * rv.castTo("int").getInt());
	}
	return ret;
}
sil::Identifier sil::Identifier::operator /(Identifier& rv) {
	Identifier ret;
	ret.setDef(Definition::constant);
	if (type == "double" || rv.type == "double") {
		ret.setType("double");
		ret.setDouble(this->castTo("double").getDouble() / rv.castTo("double").getDouble());
	} else if (type == "int" || rv.type == "int") {
		ret.setType("int");
		ret.setInt(this->castTo("int").getInt() / rv.castTo("int").getInt());
	}
	return ret;
}
sil::Identifier sil::Identifier::operator &&(Identifier& rv) {
	Identifier ret;
	return ret.setDef(Definition::constant).setType("bool")
		.setBool(this->castTo("bool").getBool() && rv.castTo("bool").getBool());
}
sil::Identifier sil::Identifier::operator ||(Identifier& rv) {
	Identifier ret;
	return ret.setDef(Definition::constant).setType("bool")
		.setBool(this->castTo("bool").getBool() || rv.castTo("bool").getBool());
}

// Expression class
sil::Expression::Expression() {
	ident = 0;
}
sil::Expression::Expression(sil::IdentRefId ip) {
	ident = ip;
}
sil::Expression::Expression(std::vector<Expression> vexprs) {
	exprs = vexprs;
}
bool sil::Expression::isIdentifier() {
	return ident;
}
/*
sil::Expression& sil::Expression::setIdentifier(sil::Identifier& v) {
	ident = &v;
	return *this;
}
sil::Expression& sil::Expression::setIdentifier(sil::Identifier* v) {
	ident = v;
	return *this;
}*/
sil::IdentRefId sil::Expression::getIdentRefId() {
	return ident;
}
sil::Expression& sil::Expression::pushExpression(sil::Expression expr) {
	if (0 < ident) {
		exprs.push_back(Expression(ident));
		ident = 0;
	}
	exprs.push_back(expr);
	return *this;
}
std::vector<sil::Expression>& sil::Expression::getExpressions() {
	return exprs;
}
std::string sil::Expression::expressionTree(Interpreter& rs) {
	return expressionTree(rs, 0);
}
std::string sil::Expression::expressionTree(Interpreter& rs, int depth) {
	std::string ret;
	for (int i = 0; i < depth; i++) ret += "\t";
	if (isIdentifier()) {
		if (rs.getIdentifier(ident).isConstant()) {
			if (rs.getIdentifier(ident).getType() == "string") ret += "String: ";
			else if (rs.getIdentifier(ident).getType() == "int" || rs.getIdentifier(ident).getType() == "double") ret += "Number: ";
			else ret += "Identifier(Constant): ";
			ret += rs.getIdentifier(ident).castTo("string").getString();
		} else {
			ret += "Identifier: " + rs.getIdentifier(ident).getSurface();
		}
		ret += "\n";
	} else {
		ret += "Expression: \n";
		for (sil::Expression e : exprs) {
			ret += e.expressionTree(rs, depth + 1);
		}
	}
	return ret;
}

// Statement class
sil::Statement::Statement() {
	loopCount = 0;
}
sil::Statement::Statement(Expression vexpr) : Statement() {
	expr = vexpr;
}
sil::Statement& sil::Statement::setExpression(Expression vexpr) {
	expr = vexpr;
	return *this;
}
sil::Statement& sil::Statement::setExpression(std::vector<Expression> vexprs) {
	expr = Expression(vexprs);
	return *this;
}
sil::Expression sil::Statement::getExpression() {
	return expr;
}
sil::Statement& sil::Statement::pushStatement(Statement stmt) {
	stmts.push_back(stmt);
	return *this;
}
std::vector<sil::Statement>& sil::Statement::getStatements() {
	return stmts;
}
bool sil::Statement::isBlock() {
	return stmts.size();
}
std::string sil::Statement::statementTree(Interpreter& rs) {
	return statementTree(rs, 0);
}
std::string sil::Statement::statementTree(Interpreter& rs, int depth) {
	std::string ret;
	for (int i = 0; i < depth; i++) ret += "\t";
	if (isBlock()) {
		ret += "Block: ";
		ret += expr.expressionTree(rs, depth + 1);
		for (sil::Statement s : stmts) {
			ret += s.statementTree(rs, depth + 1);
		}
	} else {
		ret += "Statement: \n";
		ret += expr.expressionTree(rs, depth + 1);
	}
	return ret;
}


// Interpreter class
// Identifier Storage
sil::Interpreter::IdentifierStorage::IdentifierStorage() {
	this->push(Identifier().setDef(Identifier::Definition::invalid));
}
sil::IdentRefId sil::Interpreter::IdentifierStorage::push(Identifier ident) {
	if (allocStack.size() == 0) {
		idents.push_back(ident);
		return idents.size() - 1;
	} else {
		IdentRefId pos = allocStack.back();
		idents[pos] = ident;
		allocStack.pop_back();
		return pos;
	}
}
sil::Identifier& sil::Interpreter::IdentifierStorage::get(IdentRefId id) {
	if (idents.size() <= id || idents[id].isInvalid()) throw InterpreterException("Invalid identifier referenced");
	return idents[id];
}
sil::Identifier& sil::Interpreter::IdentifierStorage::operator [](IdentRefId id) {
	return get(id);
}
void sil::Interpreter::IdentifierStorage::destroy(IdentRefId id) {
	if (idents.size() <= id || idents[id].isInvalid()) throw InterpreterException("Invalid identifier referenced");
	idents[id].setDef(sil::Identifier::Definition::invalid);
	allocStack.push_back(id);
}
sil::Identifier& sil::Interpreter::getIdentifier(IdentRefId id) {
	return istore[id];
}
sil::IdentRefId sil::Interpreter::pushIdentifier(Identifier ident) {
	return istore.push(ident);
}

// Interpreter
sil::Interpreter::Interpreter() {
	currentTarget = "";
	currentScope = 0;
	validScopeDepth = 1;
	decldIdent[currentTarget].push_back(std::unordered_map<std::string, IdentRefId>());
	Identifier ident;
	// type declaration
	declareIdentifier(ident.setSurface("int").setDef(Identifier::Definition::typeName).setType("type"));
	declareIdentifier(ident.setSurface("double").setDef(Identifier::Definition::typeName).setType("type"));
	declareIdentifier(ident.setSurface("bool").setDef(Identifier::Definition::typeName).setType("type"));
	declareIdentifier(ident.setSurface("string").setDef(Identifier::Definition::typeName).setType("type"));
	declareIdentifier(ident.setSurface("array").setDef(Identifier::Definition::typeName).setType("type"));
	// function declaration
	declareIdentifier(ident.setSurface("decas").setDef(Identifier::Definition::function)
			.setType("void").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("::").setDef(Identifier::Definition::function)
			.setType("void").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("=").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(Interpreter::identCopy));
	declareIdentifier(ident.setSurface("+").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(Interpreter::identArithmetic));
	declareIdentifier(ident.setSurface("-").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(Interpreter::identArithmetic));
	declareIdentifier(ident.setSurface("*").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(Interpreter::identArithmetic));
	declareIdentifier(ident.setSurface("/").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(Interpreter::identArithmetic));
	declareIdentifier(ident.setSurface("+=").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("-=").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("*=").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("/=").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("==").setDef(Identifier::Definition::function)
			.setType("bool").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("<").setDef(Identifier::Definition::function)
			.setType("bool").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface(">").setDef(Identifier::Definition::function)
			.setType("bool").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("<=").setDef(Identifier::Definition::function)
			.setType("bool").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface(">=").setDef(Identifier::Definition::function)
			.setType("bool").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("!").setDef(Identifier::Definition::function)
			.setType("bool").setScope(0).setInfix(false).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("if").setDef(Identifier::Definition::function)
			.setType("void").setScope(0).setInfix(false).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("loop").setDef(Identifier::Definition::function)
			.setType("void").setScope(0).setInfix(false).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("{").setDef(Identifier::Definition::control)
			.setType("void").setScope(0).setInfix(false).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("}").setDef(Identifier::Definition::control)
			.setType("void").setScope(0).setInfix(false).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("print").setDef(Identifier::Definition::function)
			.setType("void").setScope(0).setInfix(false).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("println").setDef(Identifier::Definition::function)
			.setType("void").setScope(0).setInfix(false).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("!!").setDef(Identifier::Definition::function)
			.setType("any").setScope(0).setInfix(true).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("push").setDef(Identifier::Definition::function)
			.setType("array").setScope(0).setInfix(false).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("dump").setDef(Identifier::Definition::function)
			.setType("void").setScope(0).setInfix(false).setFuncPtr(nullptr));
	declareIdentifier(ident.setSurface("include").setDef(Identifier::Definition::function)
			.setType("void").setScope(0).setInfix(false).setFuncPtr(Interpreter::identInclude));
}


sil::Statement sil::Interpreter::parse(std::string stxt) {
	bool spaceRead = true, dquoted = false, squoted = false, escaped = false, inSqBracket = false;
	std::string buffer;
	std::vector<Statement> targetStmt;
	targetStmt.push_back(Statement());
	std::vector<Expression> targetExpr;
	targetExpr.push_back(Expression());
	if (stxt.back() != '\n') stxt += '\n';
	for (unsigned long i = 0; i < stxt.size(); i++) {
		char c = stxt[i], nc = stxt[i + 1];
		if (c == '\n') {
			if (dquoted || squoted)
				throw InterpreterSyntaxException("Close quotation before new line");
			if (!buffer.empty()) {
				targetExpr.back().pushExpression(Expression(istore.push(Identifier(buffer))));
				buffer.clear();
			}
			targetStmt.back().pushStatement(targetExpr.back());
			targetExpr.pop_back();
			targetExpr.push_back(Expression());

		} else if (dquoted) {
			if (c == '\\') {
				switch (nc) {
					case  'n': buffer += '\n'; break;
					case  'r': buffer += '\r'; break;
					case  't': buffer += '\t'; break;
					case '\\': buffer += '\\'; break;
					case '\"': buffer += '\"'; break;
					case '\'': buffer += '\''; break;
				}
				i++;
			} else if (c == '\"') {
				if (nc != ' ' && nc != '\t' && nc != '\n') {
					throw InterpreterSyntaxException("No space after string sequence");
				}
				dquoted = false;
				// buffer += c;
				targetExpr.back().pushExpression(Expression(istore.push(
						Identifier().setDef(Identifier::Definition::constant)
						.setType("string").setString(buffer)
						)));
				buffer.clear();
			} else buffer += c;
		} else if (squoted) {
			if (c == '\\' && nc == '\'') {
				buffer += '\'';
				i++;
			} else if (c == '\'') {
				if (nc != ' ' && nc != '\t' && nc != '\n') {
					throw InterpreterSyntaxException("No space after string sequence");
				}
				squoted = false;
				// buffer += c;
				targetExpr.back().pushExpression(Expression(istore.push(
						Identifier().setDef(Identifier::Definition::constant)
						.setType("string").setString(buffer)
						)));
				buffer.clear();
			} else buffer += c;

		} else if (c == '(') {
			targetExpr.push_back(Expression());
		} else if (c == ')') {
			targetExpr[targetExpr.size() - 2].pushExpression(targetExpr.back());
			targetExpr.pop_back();

		} else if (c == '{') {
			// ToDo: Expression check (if / while / for)
			targetStmt.push_back(Statement());
			targetStmt.back().setExpression(targetExpr.back());
			targetExpr.pop_back();
			targetExpr.push_back(Expression());
		} else if (c == '}') {
			targetStmt[targetStmt.size() - 2].pushStatement(targetStmt.back());
			targetStmt.pop_back();

		} else if (c == '\\') {
			i++;

		} else if (c == ' ' || c == '\t') {
			switch (nc) {
				case '\"': dquoted = true; i++; break;
				case '\'': squoted = true; i++; break;
			}
		} else {
			if (nc == ' ' || nc == '\t' || nc == ')') {
				buffer += c;
				targetExpr.back().pushExpression(Expression(istore.push(Identifier(buffer))));
				buffer.clear();
			} else if (c == '[') {
				targetExpr.push_back(Expression());
				targetExpr.back().pushExpression(Expression(istore.push(Identifier(buffer))));
				buffer.clear();
				targetExpr.back().pushExpression(Expression(istore.push(Identifier("!!"))));
				targetExpr.push_back(Expression());
			} else if (c == ']') {
				if (!buffer.empty()) {
					targetExpr.back().pushExpression(Expression(istore.push(Identifier(buffer))));
					buffer.clear();
				}
				targetExpr[targetExpr.size() - 2].pushExpression(targetExpr.back());
				targetExpr.pop_back();
				targetExpr[targetExpr.size() - 2].pushExpression(targetExpr.back());
				targetExpr.pop_back();
			} else {
				buffer += c;
			}
		}
	}
	return targetStmt[0];
}

sil::Statement sil::Interpreter::parseFile(std::string path) {
	std::ifstream sourcefile(path);
	if (!sourcefile.is_open()) throw InterpreterException("Specified file not found");
	std::stringstream ss;
	ss << sourcefile.rdbuf();
	sourcefile.close();
	std::string source(ss.str());
	return this->parse(source);
}


std::vector<sil::IdentRefId> sil::Interpreter::eval(Expression expr) {
	std::vector<IdentRefId> ret;
	if (expr.isIdentifier()) {
		if (istore[expr.getIdentRefId()].isUndefined()) ret.push_back(identOf(istore[expr.getIdentRefId()].getSurface()));
		else ret.push_back(expr.getIdentRefId());
		return ret;
	}
	std::vector<Expression> exprs = expr.getExpressions();
	std::vector<Expression> exprStack;
	for (int i = exprs.size() - 1; 0 <= i; i--) {
		IdentRefId ident;
		if (exprs[i].isIdentifier() && istore[(ident = eval(exprs[i])[0])].isFunction()) {
			if (istore[ident].isInfix() && exprStack.size() == 1 && 0 < i) {
				exprStack.push_back(exprs[--i]);
				exprStack.push_back(eval(exprs[i + 1])[0]);
			} else {
				exprStack.push_back(eval(exprs[i])[0]);
			}
			std::reverse(exprStack.begin(), exprStack.end());
			std::vector<IdentRefId> retTmp = istore[ident].callFunc(*this, exprStack);
			exprStack.clear();
			for (int j = retTmp.size() - 1; 0 <= j; j--) {
				exprStack.push_back(Expression(retTmp[j]));
			}
		} else {
			exprStack.push_back(exprs[i]);
		}
	}
	for (Expression tmpExpr : exprStack) {
		if (tmpExpr.isIdentifier()) {
			ret.push_back(tmpExpr.getIdentRefId());
		} else {
			std::vector<IdentRefId> tmpIdents = eval(tmpExpr);
			for (int i = tmpIdents.size() - 1; 0 <= i; i--) {
				ret.push_back(tmpIdents[i]);
			}
		}
	}
	std::reverse(ret.begin(), ret.end());
	return ret;
}
std::vector<sil::IdentRefId> sil::Interpreter::callFunc(std::vector<Expression> exprs) {
	if (!exprs[0].isIdentifier()) throw InterpreterRuntimeException("Unknown error at silinterpreter.cpp:" + std::to_string(__LINE__));
	Identifier& ident = istore[exprs[0].getIdentRefId()];
	if (!ident.isFunction()) throw InterpreterRuntimeException("Unknown error at silinterpreter.cpp:" + std::to_string(__LINE__));

	return ident.callFunc(*this, exprs);
}
std::vector<sil::IdentRefId> sil::Interpreter::callBuiltinFunc(std::vector<Expression> exprs) {
	Identifier& ident = istore[exprs[0].getIdentRefId()];

	std::vector<IdentRefId> ret;
	std::string surface = ident.getSurface();
	if (surface == "decas" || surface == "::") {
		std::vector<IdentRefId> lv = eval(exprs[1]);
		std::vector<IdentRefId> rv = eval(exprs[2]);
		if (rv.size() != 1) throw InterpreterRuntimeException("rvalue size != 1");
		if (!istore[rv[0]].isType()) throw InterpreterRuntimeException("rvalue is not type");
		if (lv.size() < 1) throw InterpreterRuntimeException("lvalue size smaller < 1");
		Identifier& rvi = istore[rv[0]];
		for (IdentRefId ip : lv) {
			Identifier ident = istore[ip];
			ident.setDef(Identifier::Definition::variable).setType(rvi).setScope(currentScope);
			IdentRefId tmpi = declareIdentifier(ident);
			if (!tmpi) throw InterpreterRuntimeException("Redefinition not supported");
			ret.push_back(tmpi);
		}
	} else if (surface == "print" || surface == "println") {
		for (unsigned int i = 1; i < exprs.size(); i++) {
			for (IdentRefId tmpi : eval(exprs[i])) {
				std::cout << istore[tmpi].castTo("string").getString();
			}
		}
		if (surface == "println") std::cout << std::endl;
		else std::cout << std::flush;
	} else if (surface == "!!") {
		std::vector<IdentRefId> lv = eval(exprs[1]);
		std::vector<IdentRefId> rv = eval(exprs[2]);
		if (lv.size() != rv.size()) throw InterpreterRuntimeException("lvalue size not equal to rvalue size");
		for (unsigned int i = 0; i < lv.size(); i++) {
			if (istore[lv[i]].getType() == "array") {
				//ret.push_back(&lv[i]->getArray(istore[rv[i]].castTo("int").getInt()));
			} else if (istore[lv[i]].getType() == "map") {
				//ret.push_back(&lv[i]->getMap(rv[i]->castTo("string").getString()));
			}
		}
	} else if (surface == "dump") {
		for (unsigned int i = 1; i < exprs.size(); i++) {
			for (IdentRefId tmpi : eval(exprs[i])) {
				std::cout << "(id: " << tmpi << ", " << istore[tmpi].getSurface() << " :: " << istore[tmpi].getType() <<
					" = " << istore[tmpi].castTo("string").getString() << ") ";
			}
			std::cout << std::endl;
		}
	}
	return ret;
}
std::vector<sil::IdentRefId> sil::Interpreter::identInclude(Interpreter& rs, std::vector<Expression> exprs) {
	std::vector<IdentRefId> ret;
	for (unsigned int i = 1; i < exprs.size(); i++) {
		std::vector<IdentRefId> tgtLib = rs.eval(exprs[i]);
		std::string libFile = rs.getIdentifier(tgtLib[0]).getString();
		std::string asName = (2 <= tgtLib.size()) ? rs.getIdentifier(tgtLib[1]).getString() : libFile;
		rs.includeLib(libFile, asName, exprs);
	}
	return ret;
}
std::vector<sil::IdentRefId> sil::Interpreter::identCopy(std::vector<IdentRefId>& lv, std::vector<IdentRefId>& rv) {
	std::vector<IdentRefId> ret;
	if (lv.size() != rv.size()) throw InterpreterRuntimeException("lvalue count not equal to rvalue count");
	for (unsigned int i = 0; i < lv.size(); i++) {
		if (getIdentifier(lv[i]).getType() == "array") {
		} else if (getIdentifier(lv[i]).getType() == "map") {
		} else {
			istore[lv[i]].setValue(istore[rv[i]].castTo(istore[lv[i]].getType()).getValue());
			ret.push_back(lv[i]);
		}
	}
	return ret;
}
std::vector<sil::IdentRefId> sil::Interpreter::identCopy(Interpreter& rs, std::vector<Expression> exprs) {
	if (exprs.size() != 3) throw InterpreterRuntimeException("arguments count too few/many");
	std::vector<IdentRefId> lv = rs.eval(exprs[1]);
	std::vector<IdentRefId> rv = rs.eval(exprs[2]);
	return rs.identCopy(lv, rv);
}
std::vector<sil::IdentRefId> sil::Interpreter::identArithmetic(Interpreter& rs, std::vector<Expression> exprs) {
	enum basicOperator {
		add,
		sub,
		mul,
		div,
		other
	};
	std::vector<IdentRefId> ret;
	Identifier& opid = rs.getIdentifier(exprs[0].getIdentRefId());
	std::string opStr = opid.getSurface();
	basicOperator opr = 
		(opStr == "+") ? add :
		(opStr == "-") ? sub :
		(opStr == "*") ? mul :
		(opStr == "/") ? div : other;

	std::vector<IdentRefId> lv = rs.eval(exprs[1]);
	std::vector<IdentRefId> rv = rs.eval(exprs[2]);
	if (lv.size() != rv.size()) throw InterpreterRuntimeException("lvalue size not equal to rvalue size");
	for (unsigned int i = 0; i < lv.size(); i++) {
		switch (opr) {
			case add:
				ret.push_back(rs.pushIdentifier(rs.getIdentifier(lv[i]) + rs.getIdentifier(rv[i])));
				break;
			case sub:
				ret.push_back(rs.pushIdentifier(rs.getIdentifier(lv[i]) - rs.getIdentifier(rv[i])));
				break;
			case mul:
				ret.push_back(rs.pushIdentifier(rs.getIdentifier(lv[i]) * rs.getIdentifier(rv[i])));
				break;
			case div:
				ret.push_back(rs.pushIdentifier(rs.getIdentifier(lv[i]) / rs.getIdentifier(rv[i])));
				break;
			default: break;
		}
	}
	return ret;
}


int sil::Interpreter::run(Statement stmt) {
	if (stmt.isBlock()) {
		currentScope++;
		validScopeDepth++;
		decldIdent[currentTarget].push_back(std::unordered_map<std::string, IdentRefId>());
		for (Statement s : stmt.getStatements()) {
			run(s);
		}
		currentScope--;
		validScopeDepth--;
		for (auto itr = decldIdent[currentTarget].back().begin(); itr != decldIdent[currentTarget].back().end(); itr++) {
			istore.destroy(itr->second);
		}
		decldIdent[currentTarget].pop_back();
	} else {
		eval(stmt.getExpression());
	}
	return 0;
}
int sil::Interpreter::run(std::string str) {
	run(parse(str));
	return 0;
}


sil::IdentRefId sil::Interpreter::declareIdentifier(Identifier ident) {
	if (!decldIdent[currentTarget][ident.getScope()].count(ident.getSurface())) {
		decldIdent[currentTarget][ident.getScope()][ident.getSurface()] = istore.push(ident);
		return decldIdent[currentTarget][ident.getScope()][ident.getSurface()];
	} else if (istore[decldIdent[currentTarget][ident.getScope()][ident.getSurface()]].isUndefined()) {
		istore[decldIdent[currentTarget][ident.getScope()][ident.getSurface()]] = ident;
		return decldIdent[currentTarget][ident.getScope()][ident.getSurface()];
	} else {
		return 0;
	}
}
/*
sil::Identifier* sil::Interpreter::tmpIdentifier(Identifier ident) {
	Identifier& tmpi = (*std::next(decldIdent[""].begin(), ident.getScope()))["(tmp)"];
	if (tmpi.getType() != "array") tmpi.setType("array");
	tmpi.getArray().push_back(ident);
	return &tmpi.getArray().back();
}*/
bool sil::Interpreter::isIdentifierDeclared(std::string target, int scope, int scopeDepth, std::string is) {
	auto targetItr = std::next(decldIdent[target].begin(), scope);
	for (int i = 0; i < scopeDepth; i++, targetItr--) {
		int j = scope - i;
		if (targetItr->count(is)) return true;
	}
	return false;
}
bool sil::Interpreter::isIdentifierDeclared(std::string target, int scope, std::string is) {
	return isIdentifierDeclared(target, scope, validScopeDepth, is);
}
bool sil::Interpreter::isIdentifierDeclared(std::string target, std::string is) {
	return isIdentifierDeclared(target, currentScope, is);
}
bool sil::Interpreter::isIdentifierDeclared(std::string is) {
	return isIdentifierDeclared(currentTarget, is);
}
sil::IdentRefId sil::Interpreter::identOf(std::string target, int scope, int scopeDepth, std::string is) {
	for (int i = 0; i < scopeDepth; i++) {
		int j = scope - i;
		if (decldIdent[target][j].count(is)) return decldIdent[target][j][is];
	}
	IdentRefId newid = istore.push(Identifier().setSurface(is).setScope(scope));
	decldIdent[target][scope][is] = newid;
	return newid;
}
sil::IdentRefId sil::Interpreter::identOf(std::string target, int scope, std::string is) {
	return identOf(target, scope, validScopeDepth, is);
}
sil::IdentRefId sil::Interpreter::identOf(std::string target, std::string is) {
	return identOf(target, currentScope, is);
}
sil::IdentRefId sil::Interpreter::identOf(std::string is) {
	auto dpos = is.find_first_of(".");
	if (dpos != std::string::npos) {
		return identOf(is.substr(0, dpos), 0, 1, is.substr(dpos + 1));
	}
	return identOf(currentTarget, is);
}

sil::Interpreter& sil::Interpreter::includeLib(std::string file, std::string as, std::vector<Expression>& args) {
	std::string tgtBkup = currentTarget;
	void *solib = dlopen((file + ".so").c_str(), RTLD_NOW);
	char* errmsg;
	if ((errmsg = dlerror()) != NULL) {
		throw InterpreterRuntimeException(std::string("Library file couldn't open.\n") + errmsg);
	}
	silLibLoader silLoadLib = (silLibLoader)dlsym(solib, "silLoadLib");
	if ((errmsg = dlerror()) != NULL) {
		throw InterpreterRuntimeException(std::string("Library function couldn't load.\n") + errmsg);
	}
	currentTarget = as;
	decldIdent[currentTarget].push_back(std::unordered_map<std::string, IdentRefId>());
	silLoadLib(*this, args);
	currentTarget = tgtBkup;
	return *this;
}


sil::InterpreterException::InterpreterException(std::string msg) : std::runtime_error(msg) {
}
sil::InterpreterSyntaxException::InterpreterSyntaxException(std::string msg) :
	InterpreterException("Syntax error: " + msg) {
}
sil::InterpreterSyntaxException::InterpreterSyntaxException(std::string msg, int line, int col) :
	InterpreterSyntaxException("line " + std::to_string(line) + ", col " + std::to_string(col) + "\n" + msg) {
}
sil::InterpreterRuntimeException::InterpreterRuntimeException(std::string msg) :
	InterpreterException("Runtime error: " + msg) {
}
sil::InterpreterRuntimeException::InterpreterRuntimeException(std::string msg, int line, int col) :
	InterpreterRuntimeException("line " + std::to_string(line) + ", col " + std::to_string(col) + "\n" + msg) {
}
