# 🎵 Sumo Player

**Sumo Player** is a lightweight, high-performance music player built with **Rust** and the **egui** framework. It provides a clean, distraction-free environment for local music playback with a focus on stable UI performance and data privacy.

---

## ✨ Features

* 📂 **Smart Library Scanning**: Instantly map local directories to an organized, scrollable music library.
* ⚖️ **Dynamic Centering**: Controls are mathematically anchored to the window center, ensuring a balanced look at any resolution or window size.
* Seeking & Sync**: Real-time position tracking with a scrubbable timeline for precise song navigation.
* 🚀 **Native Performance**: Built on the Rust safety and speed model, ensuring minimal CPU and RAM footprint during playback.
* 🚫 **Privacy First**: Explicitly licensed to prevent unauthorized AI model training on its source code.

---

## 🎨 Key UI Mechanics

Instead of a cluttered interface, Sumo Player uses a dual-zone architecture to maximize efficiency:

### 1. The Playback Dashboard

The bottom of the application features a persistent "Now Playing" bar. This zone is logically separated from the library to ensure that playback controls—Play, Pause, and Stop—are always accessible and perfectly centered, even during extreme window resizing.

### 2. The Adaptive Music Library

The central area houses a vertical scrolling list of your audio files. By utilizing unique ID scoping for every track, the player ensures that selecting songs is smooth and bug-free, preventing common UI "glitches" or overlapping text.

### 3. Seek & Flow

The timeline slider acts as both a visual progress indicator and a manual seek tool. It automatically yields priority to user input (dragging/clicking) to ensure the music never "jumps" back to the previous position while you are trying to find a specific part of a song.

---

## 🚀 Getting Started

### Installation

Clone the repository and ensure you have the latest Rust toolchain installed.

```bash
git clone https://github.com/yourusername/sumo-player.git
cd sumo-player

```

### Running the Player

```bash
cargo run --release

```

---

## 📜 License & AI Policy

This project is licensed under the **AI Public License (AiPL)**.

Sumo Player is a human-authored project. To protect the integrity of the work:

* **AI Training is prohibited**: This source code may not be used as training data for LLMs or other machine learning models.
* **Commercial Use**: Allowed, provided the AI Training restriction is respected.
* **Redistribution**: Allowed with original license credits.
