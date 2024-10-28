# invaders
Space invaders clone written in Rust with macroquad.
## For the web
Build with
```
cargo build --release --target wasm32-unknown-unknown
```
Then copy `target/wasm32-unknown-unknown/release/invaders.wasm` to the same directory as index.html and run a web server there.
