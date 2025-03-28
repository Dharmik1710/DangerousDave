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
│   ├── animation.rs             # Handles animation
│   └── animation_registry.rs    # Initial animation states for different entities - dave, enemies, bullet, fire etc.
├── audio                        # (TODO) Handles game sounds & music.
├── config.rs                    # Game constants and configuration values.
├── game
│   ├── game.rs                  # Main game logic and initialization.
│   ├── game_loop.rs             # Core game loop - level init, input handling, state update and rendering
│   ├── game_manager.rs          # State updates
│   ├── game_utils.rs            # helper fun to game_manager.rs
│   ├── level.rs                 # Loads, processes, and manages game levels.
│   └── state.rs                 # Stores game state (score, lives, level data, dave, enemies, etc.).
├── input
│   ├── input_handler.rs         # Handles raw user input
│   └── player_controller.rs     # Maps inputs to player actions like movement, shooting, jumping.
├── lib.rs                       # Rust library entry point - single declarations for modules
├── main.rs                      # Entry point of the game, starts execution.
├── physics
│   ├── collisions.rs            # Provides funs related to collision detection between entities
│   ├── gravity.rs               # handles gravity
│   └── physics.rs               # Calls gravity, made it modular in order to call other functions as well
├── render
│   ├── renderer.rs              # Handles rendering logic, drawing sprites to the screen.
│   └── tile_atlas.rs            # Maps tile IDs to textures for rendering, stores texture offsets and functions
└── resources
    ├── bullet.rs                # Bullet mechanics (firing, movement, collisions).
    ├── camera.rs                # Camera system, follows player and updates view.
    ├── dave.rs                  # Main player character (Dave) logic, movement, and state.
    ├── direction.rs             # Enum for directions (left, right, up, down) and functions
    └── enemy.rs                 # Enemy behavior, movement, and attack logic.

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
