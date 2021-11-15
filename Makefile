# Polytree Session - session manager

include config.mk

SRC = main.c
OBJ = $(SRC:.c=.o)

all: options polytree-session

options:
	@echo Polytree Session build options:
	@echo "CFLAGS  = $(CFLAGS)"
	@echo "LDFLAGS = $(LDFLAGS)"
	@echo "CC      = $(CC)"

%.o: %.c
	$(CC) -c $< -o $@ $(CFLAGS)

OBJ: config.mk

polytree-session: $(OBJ)
	$(CC) -o $@ $(OBJ) $(LDFLAGS)

clean:
	rm -f polytree-session $(OBJ)

install: all
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f polytree-session $(DESTDIR)$(PREFIX)/bin
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

.PHONY: all options clean install xinstall uninstall
