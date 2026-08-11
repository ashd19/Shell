CC = cc
CFLAGS = -Wall -Wextra
SRC = src/main.c
BIN = main

all: run

$(BIN): $(SRC)
	$(CC) $(CFLAGS) -o $(BIN) $(SRC)

run: $(BIN)
	./$(BIN)

clean:
	rm -f $(BIN)
