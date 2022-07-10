set -x
set -e

cargo wasm -p resolver
cargo wasm -p controller
cargo wasm -p registry
cargo wasm -p registrar
cargo wasm -p gid
