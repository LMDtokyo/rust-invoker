# 🦀 Rust Invoker Server

High-speed, high-drama **Invoker spell-casting server**, built with [Axum](https://github.com/tokio-rs/axum), powered by magic, sweat, and a suspicious amount of coffee.

Inspired by Dota 2’s Invoker — this project challenges users to invoke spells in real-time using orb combos (`QWE`). It tracks your cast time, gives you a rank, and generates a new challenge.

---

## 🚀 Features

- ⚡ Fast API with Axum
- 🔁 Real-time spell casting challenge
- 🧠 Orb input buffer logic (`Q`, `W`, `E`)
- 🧪 Cast validation with ranking system
- 🔒 CORS-ready
- 🐳 Dockerized and production-friendly

---

## 📦 Tech Stack

- Rust 2021
- Axum + Tokio
- Tower HTTP
- Serde / JSON
- Docker (Multi-stage build)

---

## 🛠 Local Development

```bash
# Clone the repo
git clone https://github.com/LMDtokyo/rust-invoker.git
cd rust-invoker

# Build and run the server
cd server
cargo run
API starts on:
👉 http://localhost:3001
```
🐳 Docker
Build the container
```bash
docker build -t invoker-server ./server
```
Run it
```bash
docker run -p 3001:3001 invoker-server
```
🔗 API Endpoints
Method	Route	Description
```bash
GET	/spell	Get current target spell
POST	/input	Send orb input (Q, W, E)
POST	/invoke	Attempt to cast the current spell
GET	/input/buffer	View the current orb buffer
GET	/greeting	Optional greeting route
```
Project Structure
```bash
server/
├── logic/       # Spell logic, runes, rankings
├── routes/      # API routes (spell, hero, etc.)
├── state/       # App state (orb buffer, active spell)
├── services/    # Business logic (hero selection)
├── main.rs      # Entry point
├── Cargo.toml
└── Dockerfile
```
