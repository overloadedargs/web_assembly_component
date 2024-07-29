(cd adder && cargo component build --release)
(cd subtractor && cargo component build --release)
(cd calculator && cargo component build --release)
(cd command && cargo component build --release)

wac plug calculator/target/wasm32-wasip1/release/calculator.wasm --plug adder/target/wasm32-wasip1/release/adder.wasm --plug subtractor/target/wasm32-wasip1/release/subtractor.wasm -o composed.wasm
wac plug command/target/wasm32-wasip1/release/command.wasm --plug composed.wasm -o final.wasm

Deprecated
wasm-tools compose calculator/target/wasm32-wasip1/release/calculator.wasm -d adder/target/wasm32-wasip1/release/adder.wasm -o composed.wasm
wasm-tools compose command/target/wasm32-wasip1/release/command.wasm -d composed.wasm -o final.wasm

wasmtime run final.wasm 1 2 add
wasmtime run final.wasm 1 2 subtract