# Game of Life in Rust targeting WASM

To run:
0. You might need to set node options for openssl legacy providers with:
`export NODE_OPTIONS=--openssl-legacy-provider`
1. `wasm-pack build` in root directory
2. `npm init wasm-app www` to create `www` subdirectory
2. `cd www` to go into `/www` directory
3. `npm i` to install packages with npm
4. `npm run` to run a server
5. Connect to the served url