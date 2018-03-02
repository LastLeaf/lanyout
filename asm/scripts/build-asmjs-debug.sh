#!/bin/sh

EMMAKEN_CFLAGS='-s NO_EXIT_RUNTIME=1' cargo build --target=asmjs-unknown-emscripten
