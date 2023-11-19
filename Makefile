prefix ?= /usr/local
bindir = $(prefix)/bin
SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	test ! -d $(bindir) && mkdir -p $(bindir)

	install "target/release/actual" "$(bindir)/actual"
	install "units.json" "$(bindir)/units.json"
else
	install -D "target/release/actual" "$(bindir)/actual"
	install "units.json" "$(bindir)/units.json"
endif
uninstall:
	rm -rf "$(bindir)/actual"
	rm -rf "$(bindir)/units.json"
clean:
	rm -rf target
.PHONY: build install uninstall clean