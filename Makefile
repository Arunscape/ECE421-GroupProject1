all: build run clean

build: bin/Main.class bin/Shares.class bin/ShareInfo.class

bin/%.class: src/%.java
	javac -d bin --class-path bin --source-path src $<

clean:
	rm -rf bin

run:
	@cd bin && java Main

.PHONY: all build clean run
