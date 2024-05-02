#!/usr/bin/env bash
cargo build
nasm src/slim_shady.asm -f elf64
clang test.c -L./target/debug -lpamarox src/*.o -Wl,-rpath=./target/debug -nostdlib -nostdinc -g -o test
#objcopy -N rust_eh_personality target/debug/libpamarox.a


