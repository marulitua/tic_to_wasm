### Tic to WASM

Play your favorite Tic Tac Toe in web powered by [WASM](http://webassembly.org/)

## Set up rust

To generate WASM code we use [rust](https://www.rust-lang.org/en-US/)

Please use [rustup](https://www.rustup.rs/) to install rust development

Use rust nightly

```
rustup override set nightly
```

Add wasm as compile target

```
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Install [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) to bind wasm <-> js

```
cargo install wasm-bindgen-cli
```

and then

```
./build.sh
```
