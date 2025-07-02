# Cactpot Solver

[**View the live site on GitHub Pages**](https://marhag87.github.io/cactpot/)

This project was created because I wanted to try out [Cursor](https://www.cursor.com/), an AI-powered code editor. It's a solver and helper for the Mini Cactpot game from Final Fantasy XIV, built using [Rust](https://www.rust-lang.org/), [Yew](https://yew.rs/), and [Trunk](https://trunkrs.dev/).

## Setup

1. **Install Rust**
   - [Get Rust here](https://rustup.rs/)

2. **Install Trunk**
   - Recommended: [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
     ```sh
     cargo binstall --no-confirm trunk
     ```
   - Or, with Cargo:
     ```sh
     cargo install --locked trunk
     ```

3. **Install the wasm32 target**
   ```sh
   rustup target add wasm32-unknown-unknown
   ```

## Running Locally

1. Start the development server:
   ```sh
   trunk serve
   ```
   This will open the app in your browser at [http://localhost:8080/cactpot/](http://localhost:8080/cactpot/).

2. For a release build:
   ```sh
   trunk build --release
   ```
   The output will be in the `dist/` directory.
