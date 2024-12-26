rwildcard = $(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

RM_RF := rm -rf
MKDIR_P := mkdir -p
TARGET := elf64
ifeq ($(OS), Windows_NT)
	RM_RF := rmdir /s /q
	MKDIR_P := mkdir
	TARGET := win64
endif

DBG_OPTIONS ?= -g -F dwarf
GNU ?=

NASM := nasm
GCC := $(GNU)gcc
AR := $(GNU)ar

SRCDIR := src/asm
DIROUT := lib
DIROBJ := obj

SRCS = $(call rwildcard, $(SRCDIR) $(INCDIRS), *.asm)
BASEFILES = $(basename $(notdir $(SRCS)))
OBJS = $(call rwildcard, $(DIROBJ), *.o)

.DEFAULT_GOAL := all

all: clean pre-config build
.PHONY: all

build:
	@for file in $(BASEFILES); do \
		$(NASM) $(SRCDIR)/$$file.asm -o $(DIROBJ)/$$file.o -f $(TARGET) $(DBG_OPTIONS); \
		$(AR) rcs lib/libgetcwd.a obj/getcwd.o; \
	done
.PHONY: build

pre-config:
	@if [ ! -d "$(DIROBJ)" ]; then \
		$(MKDIR_P) $(DIROBJ); \
	fi

	@if [ ! -d "$(DIROUT)" ]; then \
		$(MKDIR_P) $(DIROUT); \
	fi
.PHONY: pre-config

clean:
	@$(RM_RF) $(DIROUT) $(DIROBJ)
.PHONY: clean