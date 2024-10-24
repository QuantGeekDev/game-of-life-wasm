# Game of Life in Rust targeting WASM
To run:

Note: You might need to set node options for openssl legacy providers with:
`export NODE_OPTIONS=--openssl-legacy-provider`
1. `wasm-pack build` in root directory
2. `npm init wasm-app www` to create `www` subdirectory
2. `cd www` to go into `/www` directory
3. `npm i` to install packages with npm
4. `npm run` to run a server
5. Connect to the served url
   
![ezgif-6-bb62b2bf87](https://github.com/user-attachments/assets/87ce908a-e0a2-44e0-a170-918e7b5b6520)
