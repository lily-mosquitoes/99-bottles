all: rust

rust: $(wildcard *.rs)
	mkdir -p build
	rustc $< -o build/$(basename $<)

clean:
	rm -f build/*

.PHONY: all clean
