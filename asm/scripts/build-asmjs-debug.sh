#!/bin/sh

EMMAKEN_CFLAGS='-s NO_EXIT_RUNTIME=1 --js-library ../js/src/lib/index.js' cargo build --target=asmjs-unknown-emscripten
