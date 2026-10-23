prefix ?= /usr/local
bindir = $(prefix)/bin
mandir = $(prefix)/share/man

SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	install "target/release/rcheckbook" "$(bindir)/rcheckbook"

	test ! -d $(mandir)/man1 && mkdir -p $(mandir)/man1
	install "man/rcheckbook.1" "$(mandir)/man1/rcheckbook.1"
	install "man/rcheckbook-add.1" "$(mandir)/man1/rcheckbook-add.1"
	install "man/rcheckbook-export.1" "$(mandir)/man1/rcheckbook-export.1"
	install "man/rcheckbook-import.1" "$(mandir)/man1/rcheckbook-import.1"
	install "man/rcheckbook-list.1" "$(mandir)/man1/rcheckbook-list.1"
	install "man/rcheckbook-remove.1" "$(mandir)/man1/rcheckbook-remove.1"
	install "man/rcheckbook-summary.1" "$(mandir)/man1/rcheckbook-summary.1"
	install "man/rcheckbook-update.1" "$(mandir)/man1/rcheckbook-update.1"
else
	install -D "target/release/rcheckbook" "$(bindir)/rcheckbook"
	install "man/rcheckbook.1" "$(mandir)/man1/rcheckbook.1"
	install "man/rcheckbook-add.1" "$(mandir)/man1/rcheckbook-add.1"
	install "man/rcheckbook-export.1" "$(mandir)/man1/rcheckbook-export.1"
	install "man/rcheckbook-import.1" "$(mandir)/man1/rcheckbook-import.1"
	install "man/rcheckbook-list.1" "$(mandir)/man1/rcheckbook-list.1"
	install "man/rcheckbook-remove.1" "$(mandir)/man1/rcheckbook-remove.1"
	install "man/rcheckbook-summary.1" "$(mandir)/man1/rcheckbook-summary.1"
	install "man/rcheckbook-update.1" "$(mandir)/man1/rcheckbook-update.1"
endif
uninstall:
	rm -rf "$(bindir)/rcheckbook"
	rm "$(mandir)/man1/rcheckbook.1"
	rm "$(mandir)/man1/rcheckbook-add.1"
	rm "$(mandir)/man1/rcheckbook-export.1"
	rm "$(mandir)/man1/rcheckbook-import.1"
	rm "$(mandir)/man1/rcheckbook-list.1"
	rm "$(mandir)/man1/rcheckbook-remove.1"
	rm "$(mandir)/man1/rcheckbook-summary.1"
	rm "$(mandir)/man1/rcheckbook-update.1"
clean:
	rm -rf target
.PHONY: build install uninstall clean