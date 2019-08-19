CC = g++
DEBUG = -DDEBUG -g3
LIBRARYOPT = -shared -fPIC
COPTION = -std=c++11 $(DEBUG)

all: interpreter libraries

interpreter:
	$(CC) $(COPTION) -ldl silinterpreter.cpp sil.cpp -o sil

libraries:
	$(CC) $(COPTION) $(LIBRARYOPT) silmath.cpp -o math.so

clean:
	rm sil
