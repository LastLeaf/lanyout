#!/bin/sh

EMMAKEN_CFLAGS='--llvm-lto 3 -O3 -Os --closure 1 -s NO_EXIT_RUNTIME=1 --js-library ../lib/interfaces.js' cargo build --target=asmjs-unknown-emscripten --release
