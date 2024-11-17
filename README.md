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
cargo build --release --features audio,touch --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/invaders.wasm .
cargo build --release --features audio,touch,wav --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/invaders.wasm ./invaders-wav.wasm
```
Then copy `invaders.wasm` and `invaders-wav.wasm` to the same directory as index.html and run a web server there.
## Features
- `audio`: enables sound
- `touch`: enables touchscreen controls
- `wav`: includes wav audio instead of OGG for safari support
