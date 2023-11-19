prefix ?= /usr/local
bindir = $(prefix)/bin
resourcedir = /opt/rcheckbook
SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	test ! -d $(resourcedir) && mkdir -p $(resourcedir)

	install "target/release/rcheckbook" "$(bindir)/rcheckbook"
	install "register.db" "$(resourcedir)/register.db"
else
	install -D "target/release/rcheckbook" "$(bindir)/rcheckbook"
	install "register.db" "$(resourcedir)/register.db"
endif
uninstall:
	rm -rf "$(bindir)/rcheckbook"
	rm -rf "$(resourcedir)"
clean:
	rm -rf target
.PHONY: build install uninstall clean