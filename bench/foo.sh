#!/bin/sh

set -ex

rustc gen.rs
./gen > to_bench.wat
wasm-tools parse to_bench.wat -o to_bench.wasm
cargo build --release --target wasm32-wasip1

before=wasmtime
after=/home/alex/code/wasmtime/target/release/wasmtime
wasm=./target/wasm32-wasip1/release/wat.wasm

$before --version
$after --version

$before run -Ccache=n --dir . --preload to_bench=./to_bench.wat $wasm --bench --list
$after run -Ccache=n --dir . --preload to_bench=./to_bench.wat $wasm --bench --list
node run-node.js $wasm --bench --list

echo =========================================================================
echo NODE
echo =========================================================================
# node run-node.js $wasm --bench --save-baseline node

echo =========================================================================
echo BEFORE
echo =========================================================================

# $before run -Ccache=n --dir . --preload to_bench=./to_bench.wat $wasm --bench \
#   --baseline node

echo =========================================================================
echo AFTER
echo =========================================================================

$after run -Ccache=n --dir . --preload to_bench=./to_bench.wat $wasm --bench \
  --baseline node 0
