### Web Assembly Component

This repository demonstrates the latest steps for compiling multiple web assembly components to be used on the command line. It was originally going to include a web server as well.

## Building
`(cd adder && cargo component build --release)`

`(cd subtractor && cargo component build --release)`

`(cd calculator && cargo component build --release)`

`(cd command && cargo component build --release)`

## Composing
`wac plug calculator/target/wasm32-wasip1/release/calculator.wasm --plug adder/target/wasm32-wasip1/release/adder.wasm --plug subtractor/target/wasm32-wasip1/release/subtractor.wasm -o composed.wasm`

`wac plug command/target/wasm32-wasip1/release/command.wasm --plug composed.wasm -o final.wasm`

## Running
`wasmtime run final.wasm interest_rate 1 2 3`

`wasmtime run final.wasm add 1 2`

`wasmtime run final.wasm subtract 1 2 subtract`

could add wac compile script


## Deprecated compose
`wasm-tools compose calculator/target/wasm32-wasip1/release/calculator.wasm -d adder/target/wasm32-wasip1/release/adder.wasm -o composed.wasm`

`wasm-tools compose command/target/wasm32-wasip1/release/command.wasm -d composed.wasm -o final.wasm`
