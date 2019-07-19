CC = g++
DEBUG = -DDEBUG -g3
COPTION = -std=c++11 $(DEBUG)

all: interpreter

interpreter:
	$(CC) $(COPTION) interpreter.cpp test.cpp -o interpreter

clean:
	rm interpreter
