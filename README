# Dangerous Dave - Rust Edition

Welcome to **Dangerous Dave in Rust!** 🎮 This is a faithful re-implementation of the classic DOS-era platformer, **Dangerous Dave**, written in Rust with SDL2.

## 📜 About the Project

This project recreates the legendary *Dangerous Dave* using Rust, featuring smooth animations, pixel-perfect physics, and modular design. Everything from **jump physics** to **enemy AI** is built from scratch while keeping it true to the original.

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

| Key | Action |
|------|-------------|
| `A` / `Left Arrow` | Move Left |
| `D` / `Right Arrow` | Move Right |
| `W` / `Up Arrow` | Jump |
| `S` / `Down Arrow` | Move Down (when using jetpack) |
| `Left Ctrl` | Shoot |
| `Left Alt` | Toggle Jetpack |
| `Esc` / `Q` | Quit |

---

## 📁 Project Structure

Here's how things are organized:

```
📂 src
 ├── 📂 game
 │   ├── dave.rs  # Dave's movement, jumping, physics
 │   ├── enemy.rs  # Enemy behavior & AI
 │   ├── level.rs  # Level loading, tile updates
 │   ├── camera.rs # Camera logic (scrolling)
 │   ├── state.rs  # Game state, tracking score, lives, etc.
 │   └── collectibles.rs # Managing pickups (trophy, gun, etc.)
 │
 ├── 📂 physics
 │   ├── physics.rs  # Manages physics engine
 │   ├── gravity.rs  # Handles gravity & jumping logic
 │   ├── collisions.rs # Detects collisions with walls, enemies, collectibles
 │
 ├── 📂 input
 │   ├── input_handler.rs  # Keyboard input tracking
 │   ├── player_controller.rs # Handles movement & shooting
 │
 ├── 📂 render
 │   ├── renderer.rs # Handles all rendering
 │   ├── animations.rs # Deals with sprite animations
 │   ├── tile_atlas.rs # Stores tile textures
 │
 ├── 📂 resources
 │   ├── direction.rs # Enum for movement directions
 │   ├── config.rs # Global constants (scale, gravity, speeds, etc.)
 │
 ├── 📂 assets
 │   ├── levels/  # Level files
 │   ├── sprites/ # Game textures & sprites
 │
 ├── Cargo.toml  # Dependencies & project metadata
 └── main.rs  # The entry point for the game
```

---

## ⚙️ Configuration
All the important **game parameters** (gravity, speed, jump height, etc.) are in `config.rs`. Feel free to tweak them to change how Dave moves!

Some key ones:
```rust
pub static SCALE: u32 = 6;
pub static GAME_TILE_SIZE: u32 = 16 * SCALE;
pub static GRAVITY: i32 = 2;
pub static DAVE_INITIAL_VELOCITY: i32 = -24;
pub static DAVE_MAX_FALL_SPEED: i32 = 18;
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
- [ ] Improve AI movement & shooting patterns
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
This project is open-source under the **MIT License**. Feel free to fork, modify, and contribute!

🚀 **Happy gaming!** 🎮

