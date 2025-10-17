# Juice: Zero Bugs Given

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Powered by Bevy](https://img.shields.io/badge/Powered%20by-Bevy-blue?logo=bevy)](https://bevyengine.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<img width="1392" height="860" alt="Screenshot 2025-08-28 at 10 08 21" src="https://github.com/user-attachments/assets/ccddc26f-4967-428d-a8cd-e5d3f22b9d9e" />

**Juice: Zero Bugs Given** is a Rust-powered 2D fighter game where you play as *Juice*, battling bosses inspired by infamous programming nightmares. Each battle represents a classic bug that Rust was designed to defeat, blending coding humor with arcade-style action.

---

## 🚧 Project Status

This game is currently in **active development**. The core 1v1 mechanics are in place, but new features, "bug" bosses, and polish are being added.

---

## 🎮 Features

* Play as **Juice**, the fearless fighter.
* Local 1v1 multiplayer support.
* Fight bosses modeled after common programming issues:
    * **Null Pointer** – vanishes into nothingness.
    * **Undefined Behavior** – glitchy, unpredictable attacks.
    * **Data Race** – fights you in parallel with a clone.
    * **Use After Free** – keeps coming back after defeat.
    * **Buffer Overflow** – grows uncontrollably large.
* Built with **Rust** for safety and performance.

---

## 🕹️ How to Play

### Controls

| Action | Player 1 | Player 2 |
| :--- | :---: | :---: |
| Move Left | `A` | `←` (Left Arrow) |
| Move Right | `D` | `→` (Right Arrow) |
| Attack | `F` | `L` |

**Global Controls:**
* **Escape:** Pause / Resume Game
* **Space (on Game Over screen):** Return to Main Menu

---

## 🛠 Tech Stack

* **Rust** as the core language.
* [**Bevy**](https://bevyengine.org/) as the 2D game engine.
* [**bevy_xpbd**](https://github.com/jshvrsn/bevy_xpbd) for 2D physics.
* **Cargo** for dependency management.

---

## 🚀 Getting Started

### Prerequisites

1.  **Rust Toolchain**: You must have the Rust toolchain installed. You can get it from [rustup.rs](https://rustup.rs/).
2.  **OS Dependencies**: Bevy has some [OS-specific dependencies](https://bevyengine.org/learn/book/getting-started/setup/#install-os-dependencies) (e.g., for audio and windowing on Linux). Please install them for your platform.

### Installation & Running

1.  Clone the repo:
    ```bash
    git clone [https://github.com/josiah-mbao/Juice-Zero-Bugs-Given.git](https://github.com/josiah-mbao/Juice-Zero-Bugs-Given.git)
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

---

## 🤝 Contributing

Contributions are welcome! If you have an idea for a new 'bug' boss (like a `Segfault` or `Off-by-One`) or want to fix an issue, feel free to fork the repo and submit a pull request.

---

## 📝 License

This project is licensed under the MIT License. See the `LICENSE` file for details.
