# Instagram Padding Tool in WASM

A simple WebAssembly experiment to create borders around images taken in different aspect ratios in order to make them 1:1 square format.

## Run the project
1. Clone the project
2. Have Rust stable toolchain and Node installed
3. Run following commands:
    ```sh
    npm install
    npm run wasm
    npm run dev
    ```

## Netlify build command
```sh
rustup toolchain install stable \ 
    && cargo install wasm-pack \ 
    && npm run wasm-release \ 
    && npm run build-only
```