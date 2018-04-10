### Tic to WASM

Play your favorite Tic Tac Toe in web powered by [WASM](http://webassembly.org/)

## Set up rust

To generate WASM code we use [rust](https://www.rust-lang.org/en-US/)

Please use [rustup](https://www.rustup.rs/) to install rust development

We also need to use rust nightly

```
rustup override set nightly
```

We need to add wasm as compile target

```
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## Set up npm

```
npm i
```

for development

```
npm run dev
```

for production

```
npm run pro
```
