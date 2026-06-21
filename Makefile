prefix ?= /usr/local
bindir = $(prefix)/bin
mandir = $(prefix)/share/man

ifeq ($(REGISTRY_SCHEMA_DIR),)
resourcedir = $(prefix)/share/rcheckbook
else
resourcedir = $(REGISTRY_SCHEMA_DIR)
endif

SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	test ! -d $(resourcedir) && mkdir -p $(resourcedir)

	install "target/release/rcheckbook" "$(bindir)/rcheckbook"
	install "register.sql" "$(resourcedir)/register.sql"

	test ! -d $(prefix)/share/man/man1 && mkdir -p $(prefix)/share/man/man1
	install "man/rcheckbook.1" "$(prefix)/share/man/man1/rcheckbook.1"
	install "man/rcheckbook-add.1" "$(prefix)/share/man/man1/rcheckbook-add.1"
	install "man/rcheckbook-export.1" "$(prefix)/share/man/man1/rcheckbook-export.1"
	install "man/rcheckbook-import.1" "$(prefix)/share/man/man1/rcheckbook-import.1"
	install "man/rcheckbook-list.1" "$(prefix)/share/man/man1/rcheckbook-list.1"
	install "man/rcheckbook-remove.1" "$(prefix)/share/man/man1/rcheckbook-remove.1"
	install "man/rcheckbook-summary.1" "$(prefix)/share/man/man1/rcheckbook-summary.1"
	install "man/rcheckbook-update.1" "$(prefix)/share/man/man1/rcheckbook-update.1"
else
	install -D "target/release/rcheckbook" "$(bindir)/rcheckbook"
	install "register.sql" "$(resourcedir)/register.sql"
	install "man/rcheckbook.1" "$(prefix)/share/man/man1/rcheckbook.1"
	install "man/rcheckbook-add.1" "$(prefix)/share/man/man1/rcheckbook-add.1"
	install "man/rcheckbook-export.1" "$(prefix)/share/man/man1/rcheckbook-export.1"
	install "man/rcheckbook-import.1" "$(prefix)/share/man/man1/rcheckbook-import.1"
	install "man/rcheckbook-list.1" "$(prefix)/share/man/man1/rcheckbook-list.1"
	install "man/rcheckbook-remove.1" "$(prefix)/share/man/man1/rcheckbook-remove.1"
	install "man/rcheckbook-summary.1" "$(prefix)/share/man/man1/rcheckbook-summary.1"
	install "man/rcheckbook-update.1" "$(prefix)/share/man/man1/rcheckbook-update.1"
endif
uninstall:
	rm -rf "$(bindir)/rcheckbook"
	rm -rf "$(resourcedir)"
clean:
	rm -rf target
.PHONY: build install uninstall clean