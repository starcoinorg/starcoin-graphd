# starcoin-graphd

A minimal all-in-one DAG viewer for the Starcoin blockchain.
Rust backend + React frontend, served on a single port and single binary.

## How It Works
- Backend: Rust with Actix-web
- Frontend: React with Vite
- `build-static.sh` compiles the frontend and copies it to `./static`

## Usage

```bash
./build-static.sh
cargo run --release --network halley

```
then open:
http://127.0.0.1:8080

