`(cd adder && cargo component build --release)`

`(cd subtractor && cargo component build --release)`

`(cd calculator && cargo component build --release)`

`(cd command && cargo component build --release)`

`wac plug calculator/target/wasm32-wasip1/release/calculator.wasm --plug adder/target/wasm32-wasip1/release/adder.wasm --plug subtractor/target/wasm32-wasip1/release/subtractor.wasm -o composed.wasm`

`wac plug command/target/wasm32-wasip1/release/command.wasm --plug composed.wasm -o final.wasm`
