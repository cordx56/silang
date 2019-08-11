#pragma once

#include <iostream>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <vector>
#include <list>
#include <iterator>
#include <unordered_map>
#include <dlfcn.h>

namespace sil {
	class Interpreter;
	class Identifier;
	class Expression;
	class Statement;

	typedef int IdentRefId;
	typedef std::vector<IdentRefId> (*silFunc)(Interpreter&, std::vector<Expression>);

	class Identifier {
	public:
		enum Definition {
			control,
			function,
			variable,
			constant,
			typeName,
			undefined,
			invalid
		};
		struct Value {
			int v_int;
			double v_double;
			std::string v_string;
			silFunc callPtr;
			std::vector<IdentRefId> array;
			std::unordered_map<std::string, IdentRefId> map;
		};

	private:
		std::string surface;
		Definition def;
		std::string type;
		int scope;
		bool infix;
		Value v;

	public:
		Identifier();
		Identifier(std::string);

		Identifier& setSurface(std::string);
		Identifier& setDef(Definition);
		Identifier& setType(std::string);
		Identifier& setType(Identifier);
		Identifier& setScope(int);
		Identifier& setInfix(bool);
		std::string getSurface();
		Definition getDef();
		std::string getType();
		int getScope();
		bool isInfix();
		bool isFunction();
		bool isVariable();
		bool isConstant();
		bool isType();
		bool isUndefined();
		bool isInvalid();

		Identifier& setValue(Value);
		Identifier& setInt(int);
		Identifier& setDouble(double);
		Identifier& setBool(bool);
		Identifier& setString(std::string);
		Identifier& setFuncPtr(silFunc);
		Value& getValue();
		int getInt();
		double getDouble();
		bool getBool();
		std::string getString();
		std::vector<IdentRefId> callFunc(Interpreter&, std::vector<Expression>);
		std::vector<IdentRefId>& getArray();
		std::unordered_map<std::string, IdentRefId>& getMap();
		IdentRefId getArray(int);
		IdentRefId getMap(std::string);

		Identifier castTo(std::string);

		std::string identifierTree();
		std::string identifierTree(int);
	};

	class Expression {
		IdentRefId ident;
		std::vector<Expression> exprs;
	public:
		Expression();
		Expression(IdentRefId);
		Expression(std::vector<Expression>);
		bool isIdentifier();
		Expression& setIdentifier(Identifier&);
		Expression& setIdentifier(IdentRefId);
		IdentRefId getIdentifier();
		Expression& pushExpression(Expression);
		std::vector<Expression>& getExpressions();
		std::string expressionTree(Interpreter&);
		std::string expressionTree(Interpreter&, int);
	};

	class Statement {
		Expression expr;
		std::vector<Statement> stmts;
		int loopCount;
	public:
		Statement();
		Statement(Expression);
		Statement& setExpression(Expression);
		Statement& setExpression(std::vector<Expression>);
		Expression getExpression();
		Statement& pushStatement(Statement);
		std::vector<Statement>& getStatements();
		bool isBlock();
		std::string statementTree(Interpreter&);
		std::string statementTree(Interpreter&, int);
	};


	class Interpreter {
	public:
		class IdentifierStorage {
		private:
			std::vector<Identifier> idents;
			std::vector<IdentRefId> allocStack;
		public:
			IdentRefId push(Identifier);
			Identifier& get(IdentRefId);
			Identifier& operator [](IdentRefId);
			void destroy(IdentRefId);
		};
	private:
		IdentifierStorage istore;
		std::unordered_map<std::string, std::vector<std::unordered_map<std::string, IdentRefId>>> decldIdent;
		std::string currentTarget;
		int currentScope;
		int validScopeDepth;
		std::vector<int> validScopeDepthStack;
	public:
		Interpreter();
		std::vector<IdentRefId> eval(Expression);
		std::vector<IdentRefId> callFunc(std::vector<Expression>);
		std::vector<IdentRefId> callBuiltinFunc(std::vector<Expression>);

		Identifier& getIdentifier(IdentRefId);
		IdentRefId declareIdentifier(Identifier);
		//Identifier* tmpIdentifier(Identifier);
		bool isIdentifierDeclared(std::string);
		bool isIdentifierDeclared(std::string, std::string);
		bool isIdentifierDeclared(std::string, int, std::string);
		bool isIdentifierDeclared(std::string, int, int, std::string);
		IdentRefId identOf(std::string);
		IdentRefId identOf(std::string, std::string);
		IdentRefId identOf(std::string, int, std::string);
		IdentRefId identOf(std::string, int, int, std::string);
		Interpreter& importFrom(std::string, std::string, std::vector<Identifier&>);

		bool isWhitespace(char);
		Statement parse(std::string);
		int run(Statement);
		int run(std::string);

		Statement parseFile(std::string);
	};



	class InterpreterException : public std::runtime_error {
	private:
	public:
		InterpreterException(std::string);
	};
	class InterpreterSyntaxException : public InterpreterException {
	private:
	public:
		InterpreterSyntaxException(std::string);
		InterpreterSyntaxException(std::string, int, int);
	};
	class InterpreterRuntimeException : public InterpreterException {
	private:
	public:
		InterpreterRuntimeException(std::string);
		InterpreterRuntimeException(std::string, int, int);
	};
}
