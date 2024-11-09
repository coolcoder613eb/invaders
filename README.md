# invaders
Space invaders clone written in Rust with macroquad.
## Play
You can run the game with
```
cargo run --release --features audio
```
Or you can play the web version at https://invaders-4lu.pages.dev/
## Build for the web
Build with
```
cargo build --release --features audio --target wasm32-unknown-unknown
```
Then copy `target/wasm32-unknown-unknown/release/invaders.wasm` to the same directory as index.html and run a web server there.
