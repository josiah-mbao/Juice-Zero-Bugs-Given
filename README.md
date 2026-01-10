# Juice: Zero Bugs Given 🕹️

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Powered by Bevy](https://img.shields.io/badge/Powered%20by-Bevy-blue?logo=bevy)](https://bevyengine.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![itch.io](https://img.shields.io/badge/Available%20on-itch.io-fa5c5c?logo=itch.io)](https://josiah-mbao.itch.io/juice-zero-bugs-given)

<img width="1392" height="860" alt="Screenshot 2025-08-28 at 10 08 21" src="https://github.com/user-attachments/assets/ccddc26f-4967-428d-a8cd-e5d3f22b9d9e" />

**Juice: Zero Bugs Given** is a small but complete 2D arcade boss-fighting game built in **Rust** using the **Bevy** engine.

You play as *Juice*, battling bosses inspired by infamous programming bugs. Each fight maps a real software failure (like Null Pointers or Data Races) to distinct AI behaviors and combat patterns.

The project focuses on tight combat, readable enemy behavior, and ECS-driven game architecture — prioritizing finish and polish over scope.

## 🔗 Links

- 🎮 Play the game on itch.io: https://josiah-mbao.itch.io/juice-zero-bugs-given
- 📖 Development write-up: [Devlog on itch.io]

## 📊 Release Status

**Current version:** v0.1.0  
A polished single-player release with complete combat, audio, UI, and game flow systems.

## 🎮 Key Features

- Single-player battles against AI bosses inspired by programming bugs
- Unique behaviors for each boss type (Null Pointer, Undefined Behavior, Data Race, Use After Free, Buffer Overflow)
- Complete combat system with jumping, blocking, attacks, and health management
- Polished audio, UI, visual effects, and particle systems
- Difficulty scaling (Easy/Normal/Hard) and intuitive controls
- Atmospheric background music and sound effects

## Screenshots / Media

<img width="1279" height="749" alt="2d figher" src="https://github.com/user-attachments/assets/4319f63e-2a65-4013-b27c-1bdc9b87bcb5" />

## 🛠 Tech Stack

* **Rust** as the core language.
* [**Bevy**](https://bevyengine.org/) as the 2D game engine.
* [**bevy_xpbd**](https://github.com/jshvrsn/bevy_xpbd) for 2D physics.
* **Cargo** for dependency management.

## 🚀 Getting Started

### Prerequisites

1.  **Rust Toolchain**: You must have the Rust toolchain installed. You can get it from [rustup.rs](https://rustup.rs/).
2.  **OS Dependencies**: Bevy has some [OS-specific dependencies](https://bevyengine.org/learn/book/getting-started/setup/#install-os-dependencies) (e.g., for audio and windowing on Linux). Please install them for your platform.

### Installation & Running

1.  Clone the repo:
    ```bash
    git clone https://github.com/josiah-mbao/Juice-Zero-Bugs-Given.git
    cd Juice-Zero-Bugs-Given
    ```

2.  Build and run the game:
    ```bash
    cargo run
    ```
    For better performance, run in release mode:
    ```bash
    cargo run --release
    ```

### Controls

| Action | Player 1 | Player 2 |
| :--- | :---: | :---: |
| Move Left | `A` | `←` (Left Arrow) |
| Move Right | `D` | `→` (Right Arrow) |
| Jump | `W` | `↑` (Up Arrow) |
| Block | `S` | `↓` (Down Arrow) |
| Light Attack | `F` | `L` |
| Heavy Attack | `R` | `O` |
| Kick | `T` | `P` |

**In-Game Controls:**
* **P** or **Click PAUSE button**: Pause game with menu options
* **Escape**: Resume from pause (keyboard alternative)
* **Space (on Game Over screen):** Return to Main Menu

## Architecture & Systems (Advanced)

See [docs/architecture.md](docs/architecture.md) for detailed technical information on combat mechanics, boss AI, visual systems, audio framework, and more.

## 🤝 Contributing

Contributions are welcome! If you have an idea for a new 'bug' boss (like a `Segfault` or `Off-by-One`) or want to fix an issue, feel free to fork the repo and submit a pull request.

**Ideas for Enhancement:**
- Add victory/defeat music and sound effects
- Implement local multiplayer support
- Add visual effects for special attacks
- Create boss unlock progression
- Add customizable controls

## 📝 License

This project is licensed under the MIT License. See the `LICENSE` file for details.
