# Polytree Session - session manager

all: target/debug/polytree-session

include config.mk

SRC = Cargo.toml src/main.rs

target/debug/polytree-session: $(SRC)
	cargo build

install: all
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f target/debug/polytree-session $(DESTDIR)$(PREFIX)/bin
	chmod 755 $(DESTDIR)$(PREFIX)/bin/polytree-session

xinstall: install
	mkdir -p $(DESTDIR)$(ICONSPREFIX)
	cp -f polytree.png $(DESTDIR)$(ICONSPREFIX)
	chmod 644 $(DESTDIR)$(ICONSPREFIX)/polytree.png
	
	mkdir -p $(DESTDIR)$(XSESSIONSPREFIX)
	cp -f polytree.desktop $(DESTDIR)$(XSESSIONSPREFIX)
	chmod 644 $(DESTDIR)$(XSESSIONSPREFIX)/polytree.desktop

uninstall:
	rm -f \
		$(DESTDIR)$(PREFIX)/bin/polytree-session \
		$(DESTDIR)$(ICONSPREFIX)/polytree.png \
		$(DESTDIR)$(XSESSIONSPREFIX)/polytree.desktop

.PHONY: all install xinstall uninstall
