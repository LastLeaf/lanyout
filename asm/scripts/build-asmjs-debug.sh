#!/bin/sh

EMMAKEN_CFLAGS='-s NO_EXIT_RUNTIME=1 --js-library ../lib/interfaces.js' cargo build --target=asmjs-unknown-emscripten
