PREFIX := "/usr/local"
BINDIR := PREFIX+"/bin"
MANDIR := PREFIX+"/share/man"

build:
	cargo build --release

[unix]
install:
	install "target/release/rcheckbook" "{{BINDIR}}/rcheckbook"

	install "man/rcheckbook.1" "{{MANDIR}}/man1/rcheckbook.1"
	install "man/rcheckbook-add.1" "{{MANDIR}}/man1/rcheckbook-add.1"
	install "man/rcheckbook-export.1" "{{MANDIR}}/man1/rcheckbook-export.1"
	install "man/rcheckbook-import.1" "{{MANDIR}}/man1/rcheckbook-import.1"
	install "man/rcheckbook-list.1" "{{MANDIR}}/man1/rcheckbook-list.1"
	install "man/rcheckbook-remove.1" "{{MANDIR}}/man1/rcheckbook-remove.1"
	install "man/rcheckbook-summary.1" "{{MANDIR}}/man1/rcheckbook-summary.1"
	install "man/rcheckbook-update.1" "{{MANDIR}}/man1/rcheckbook-update.1"

[linux]
install:
	install -D "target/release/rcheckbook" "{{BINDIR}}/rcheckbook"
	install "man/rcheckbook.1" "{{MANDIR}}/man1/rcheckbook.1"
	install "man/rcheckbook-add.1" "{{MANDIR}}/man1/rcheckbook-add.1"
	install "man/rcheckbook-export.1" "{{MANDIR}}/man1/rcheckbook-export.1"
	install "man/rcheckbook-import.1" "{{MANDIR}}/man1/rcheckbook-import.1"
	install "man/rcheckbook-list.1" "{{MANDIR}}/man1/rcheckbook-list.1"
	install "man/rcheckbook-remove.1" "{{MANDIR}}/man1/rcheckbook-remove.1"
	install "man/rcheckbook-summary.1" "{{MANDIR}}/man1/rcheckbook-summary.1"
	install "man/rcheckbook-update.1" "{{MANDIR}}/man1/rcheckbook-update.1"
uninstall:
	rm -rf "{{BINDIR}}/rcheckbook"
	rm "{{MANDIR}}/man1/rcheckbook.1"
	rm "{{MANDIR}}/man1/rcheckbook-add.1"
	rm "{{MANDIR}}/man1/rcheckbook-export.1"
	rm "{{MANDIR}}/man1/rcheckbook-import.1"
	rm "{{MANDIR}}/man1/rcheckbook-list.1"
	rm "{{MANDIR}}/man1/rcheckbook-remove.1"
	rm "{{MANDIR}}/man1/rcheckbook-summary.1"
	rm "{{MANDIR}}/man1/rcheckbook-update.1"
clean:
	rm -rf target