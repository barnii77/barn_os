rustup target add thumbv7em-none-eabihf
rustup override set nightly
rustup component add rust-src
cargo install bootimage
rustup component add llvm-tools-preview