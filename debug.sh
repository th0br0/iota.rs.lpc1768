#!/bin/bash

GDB=arm-none-eabi-gdb
GDBFLAGS=
TARGET=target/thumbv7m-none-eabi/debug/$1
$GDB    -ex "set remotetimeout 10000" \
        -ex "target extended localhost:3333" \
        -ex "monitor arm semihosting enable" \
        -ex "monitor reset halt" \
        -ex "load" \
        -ex "monitor reset init" \
	$GDBFLAGS $TARGET; 
