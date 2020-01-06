sources = $(wildcard src/*.java)
transformed = $(patsubst src/%, bin/%, $(sources))
classes = $(transformed:.java=.class)

all: build run clean

build: $(classes)

bin/%.class: src/%.java
	javac -d bin --class-path bin --source-path src $<

clean:
	rm -rf bin

run:
	@cd bin && java Main

.PHONY: all build clean run
