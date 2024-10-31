# invaders
Space invaders clone written in Rust with macroquad.
## Play
You can run the game with
```
cargo run --release
```
Or you can play the web version at https://invaders-4lu.pages.dev/
## Build for the web
Build with
```
cargo build --release --target wasm32-unknown-unknown
```
Then copy `target/wasm32-unknown-unknown/release/invaders.wasm` to the same directory as index.html and run a web server there.
