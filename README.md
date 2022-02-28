# blog os
This is an OS build following the instructions at [https://os.phil-opp.com/](https://os.phil-opp.com/)
## Prerequisites
Install these programs
* Rust and cargo
* Qemu (and have it in path too)

Install the `bootimage` tool
```
cargo install bootimage
```
Install the `llvm-tools-preview` component
```
rustup component add llvm-tools-preview
```
Run it
```
cargo run
```
or build the .bin file
```
cargo build
```