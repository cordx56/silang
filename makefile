CC = g++
DEBUG = -DDEBUG -g3
COPTION = -std=c++11 $(DEBUG)

all: interpreter

interpreter:
	$(CC) $(COPTION) silinterpreter.cpp sil.cpp -o sil

clean:
	rm sil
