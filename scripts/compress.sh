#!/bin/sh

# find $1 -name "*.wasm" -type f -printf '%p\n' -exec wasm-opt -Oz "{}" -o "{}" \;

# We don't bother to precompress and instead just leverage a proxy
# find $1 -regex ".*\.\(html\|wasm\|js\|css\)" -type f -printf "%p\n" -exec gzip -k -f -9 "{}" \;
# find $1 -regex ".*\.\(html\|wasm\|js\|css\)" -type f -printf "%p\n" -exec brotli -9 -f "{}" \;
