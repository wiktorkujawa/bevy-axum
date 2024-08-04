# Bevy sandbox

This repository is based on the [Foxtrot](https://github.com/janhohenheim/foxtrot) template, version 0.3.0. It extends the original template with a more complex asset and level generating system. The server is running with Axum. And cors is enabled, so you can access the deployed or launched server from different origins, f.e.: https://just-dev-it.com/bevy-sandbox(Website build with Nextjs and PayloadCMS)

## Scripts

### Build

The build script compiles the game for the WebAssembly target and prepares all assets for distribution. It performs the following steps:

`
cargo build --package game --release --target wasm32-unknown-unknown && \
		wasm-bindgen --out-dir dist --target web target/wasm32-unknown-unknown/release/game.wasm && \
		cp index.html dist/index.html && \
		cp -r game/assets dist/assets
`


### Run Server

To start the game server after building, use the following command:

`
    cargo run --package server
`


This `readme.md` provides a clear overview of the project's foundation, its enhancements over the original template, and instructions for the build and server scripts.