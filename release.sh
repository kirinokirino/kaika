#!/bin/bash
cargo build --release --target wasm32-unknown-unknown; 
cp target/wasm32-unknown-unknown/release/kaika.wasm ./kaika.wasm; 
wasm-snip -o kaika.wasm kaika.wasm;
wasm-opt -O3 --strip-producers --strip-debug --dce --zero-filled-memory kaika.wasm -o kaika.wasm;
stat ./kaika.wasm | rg Size;
python ./serve.py 8001
