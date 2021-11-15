# Polytree Session version
VERSION = 0.0.0

PREFIX = /usr/local
SYSPREFIX = /usr

ICONSPREFIX = $(SYSPREFIX)/share/icons
XSESSIONSPREFIX = $(SYSPREFIX)/share/xsessions

X11INC = /usr/X11R6/include
X11LIB = /usr/X11R6/lib
# FreeBSD (uncomment)
#X11INC = /usr/local/include
#X11LIB = /usr/local/lib

INCS = -I${X11INC}
LIBS = -L${X11LIB} -lX11 -lpthread

CPPFLAGS = -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_C_SOURCE=200809L -DVERSION=\"$(VERSION)\"
CFLAGS = -std=c99 -pedantic -Wall -Wextra -Os $(INCS) $(CPPFLAGS)
LDFLAGS = $(LIBS)

CC = cc
