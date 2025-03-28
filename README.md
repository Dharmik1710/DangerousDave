# Dangerous Dave - Rust Edition

Welcome to **Dangerous Dave in Rust!** 🎮 This is a faithful re-implementation of the classic DOS-era platformer, **Dangerous Dave**, written in Rust with SDL2.

## 📜 About the Project

This project recreates the legendary _Dangerous Dave_ using Rust, featuring smooth animations, pixel-perfect physics, and modular design. Everything from **jump physics** to **enemy AI** is built from scratch while keeping it true to the original.

Also, big shoutout to [MaiZure/lmdave](https://github.com/MaiZure/lmdave) for their **config parameters** and **tile extraction scripts**! The level decoding logic is based on insights from the [Dangerous Dave Level Format](https://moddingwiki.shikadi.net/wiki/Dangerous_Dave_Level_format). 🎩

---

## 🚀 Getting Started

### Prerequisites

You'll need:

- **Rust** (latest stable version)
- **SDL2** development libraries
- **Cargo** (comes with Rust)

### Installation

Clone the repo:

```sh
$ git clone https://github.com/your-username/dangerous-dave-rust.git
$ cd dangerous-dave-rust
```

Install dependencies:

```sh
$ cargo build --release
```

Run the game:

```sh
$ cargo run
```

---

## 🎮 Controls

Here's how you play:

| Key                 | Action                         |
| ------------------- | ------------------------------ |
| `A` / `Left Arrow`  | Move Left                      |
| `D` / `Right Arrow` | Move Right                     |
| `W` / `Up Arrow`    | Jump                           |
| `S` / `Down Arrow`  | Move Down (when using jetpack) |
| `Left Ctrl`         | Shoot                          |
| `Left Alt`          | Toggle Jetpack                 |
| `Esc` / `Q`         | Quit                           |

---

## 📁 Project Structure

Here's how things are organized:

```
src
├── animation
│   ├── animation.rs
│   └── animation_registry.rs
├── audio
├── config.rs
├── game
│   ├── actions.rs
│   ├── bullet.rs
│   ├── camera.rs
│   ├── collectibles.rs
│   ├── dave.rs
│   ├── enemy.rs
│   ├── game.rs
│   ├── game_loop.rs
│   ├── game_manager.rs
│   ├── game_rules.rs
│   ├── init.rs
│   ├── level.rs
│   └── state.rs
├── input
│   ├── input_handler.rs
│   ├── keyboard.rs
│   └── player_controller.rs
├── lib.rs
├── main.rs
├── physics
│   ├── collisions.rs
│   ├── gravity.rs
│   └── physics.rs
├── render
│   ├── render_utils.rs
│   ├── renderer.rs
│   └── tile_atlas.rs
├── resources
│   └── direction.rs
```

---

## 🕹️ Gameplay Features

- **Smooth character movement** (jump, fall, run, shoot!)
- **Pixel-perfect collision detection**
- **Gravity-based physics engine** (jump arcs, fall acceleration)
- **Animated enemies with movement AI**
- **Collectibles and power-ups**
- **Trophy system + level completion logic**
- **Jetpack mechanics** (toggle flight mode)
- **Classic Dangerous Dave level designs**

---

## 🎯 TODO & Future Plans

- [ ] Integrate AI in movement & shooting patterns
- [ ] Add sound effects & background music
- [ ] Implement saving & loading of high scores
- [ ] Multiplayer co-op mode? Maybe 🤔

---

## 🙌 Credits

Massive thanks to:

- [MaiZure/lmdave](https://github.com/MaiZure/lmdave) for reference **config params & tile extraction**
- [Shikadi Modding Wiki](https://moddingwiki.shikadi.net/wiki/Dangerous_Dave_Level_format) for decoding **level & monster path data**
- **Rust & SDL2 community** for their awesome support & libraries

---

## 📜 License

Feel free to fork, modify, and contribute! If possible give some credit.

🚀 **Happy gaming!** 🎮
