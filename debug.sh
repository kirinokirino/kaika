#!/bin/bash
cargo build --target wasm32-unknown-unknown; 
cp target/wasm32-unknown-unknown/debug/kaika.wasm ./kaika.wasm; 
stat ./kaika.wasm | rg Size;
python ./serve.py 8001
