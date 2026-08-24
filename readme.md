# Filler

![GitHub stars](https://img.shields.io/github/stars/Amani2812/Filler?style=for-the-badge&logo=github) ![GitHub forks](https://img.shields.io/github/forks/Amani2812/Filler?style=for-the-badge&logo=github) ![GitHub issues](https://img.shields.io/github/issues/Amani2812/Filler?style=for-the-badge&logo=github) ![Last commit](https://img.shields.io/github/last-commit/Amani2812/Filler?style=for-the-badge&logo=github)

## 📑 Table of Contents

- [Description](#description)
- [Tech Stack](#tech-stack)
- [Quick Start](#quick-start)
- [Available Scripts](#available-scripts)
- [Project Structure](#project-structure)
- [Development Setup](#development-setup)
- [Deployment](#deployment)
- [Contributors](#contributors)
- [Contributing](#contributing)

## 📝 Description

Filler — a software project built with Docker, Rust.

## 🛠️ Tech Stack

![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white) ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

## ⚡ Quick Start

```bash

# 1. Clone the repository
git clone https://github.com/Amani2812/Filler.git

# Build and run
cargo run
```

## 🚀 Available Scripts

- **build** — `cargo build`
- **run** — `cargo run`
- **test** — `cargo test`

## 📁 Project Structure

```
.
├── AUDIT.md
├── AUDIT_RUNBOOK.md
├── HOW_TO_RUN.md
├── docker_image
│   ├── Dockerfile
│   ├── linux_game_engine
│   ├── linux_robots
│   │   ├── bender
│   │   ├── h2_d2
│   │   ├── terminator
│   │   └── wall_e
│   ├── m1_game_engine
│   ├── m1_robots
│   │   ├── bender
│   │   ├── h2_d2
│   │   ├── terminator
│   │   └── wall_e
│   ├── maps
│   │   ├── map00
│   │   ├── map01
│   │   └── map02
│   └── solution
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           ├── board.rs
│           ├── game_state.rs
│           ├── grid_trait.rs
│           ├── main.rs
│           ├── piece.rs
│           ├── player.rs
│           └── strategy.rs
├── readme.md
├── result.txt
└── start.sh
```

## 🛠️ Development Setup

### Docker
1. `docker build -t my-app .`
2. `docker run -p 3000:3000 my-app`

### Rust
1. Install Rust via [rustup](https://rustup.rs/)
2. `cargo build && cargo run`

## 🚢 Deployment

### Docker
```bash
docker build -t filler .
docker run -p 3000:3000 filler
```

## 👥 Contributing

Contributions are welcome! Here's the standard flow:

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/Amani2812/Filler.git`
3. **Branch**: `git checkout -b feature/your-feature`
4. **Commit**: `git commit -m 'feat: add some feature'`
5. **Push**: `git push origin feature/your-feature`
6. **Open** a pull request

Please follow the existing code style and include tests for new behavior where applicable.
