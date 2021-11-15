# Polytree Session version
VERSION = 0.0.0

PREFIX = /usr/local
SYSPREFIX = /usr

ICONSPREFIX = $(SYSPREFIX)/share/icons
XSESSIONSPREFIX = $(SYSPREFIX)/share/xsessions

INCS =
LIBS =

CPPFLAGS = -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_POSIX_C_SOURCE=200809L -DVERSION=\"$(VERSION)\"
CFLAGS = -std=c99 -pedantic -Wall -Os $(INCS) $(CPPFLAGS)
LDFLAGS = $(LIBS)

CC = cc
