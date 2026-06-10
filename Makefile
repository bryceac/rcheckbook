prefix ?= /usr/local
bindir = $(prefix)/bin
resourcedir = $(prefix)/share/rcheckbook
SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	test ! -d $(resourcedir) && mkdir -p $(resourcedir)

	install "target/release/rcheckbook" "$(bindir)/rcheckbook"
	install "register.sql" "$(resourcedir)/register.sql"
else
	install -D "target/release/rcheckbook" "$(bindir)/rcheckbook"
	install "register.sql" "$(resourcedir)/register.sql"
endif
uninstall:
	rm -rf "$(bindir)/rcheckbook"
	rm -rf "$(resourcedir)"
clean:
	rm -rf target
.PHONY: build install uninstall clean