# SysMon 📈

SysMon is a very simple and lightweight Rust application designed to monitor and display system usage information. It offers a terminal interface for detailed visualization of these metrics in real-time.

## Overview

The application provides the following features:
- Real-time monitoring of CPU, GPU, and memory usage.
- Graphical display of usage statistics using Ratatui.
- Command-line interface for customizing the refresh rate and enabling graphical display.

## Setup

### Prerequisites

Before you begin, ensure you have the following:
- Rust and Cargo.
- An NVIDIA GPU with `nvidia-smi` installed (for GPU usage monitoring).

### Installation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/seanima9/SysMon.git
   cd SysMon
   ```

2. **Build the project**:
   ```sh
   cargo build --release
   ```

3. **Run the application**:
   ```sh
   cargo run --release
   ```

## Usage

### Command-Line Arguments

The application accepts command-line arguments to customize its behavior.

- `--refresh`: Sets the refresh rate of the UI updates in milliseconds (default is 600ms).
- `--graphs`: Enables detailed graphical display of system stats.

### Example Commands

- **Run with default settings**:
  ```sh
  cargo run --release
  ```

- **Run with a custom refresh rate**:
  ```sh
  cargo run --release -- --refresh 1000
  ```

- **Run with graphical display enabled**:
  ```sh
  cargo run --release -- --graphs
  ```

## Features

- **Real-Time Graphs**: Displays CPU, GPU, and memory usage graphs side by side.
- **Customizable Refresh Rate**: Allows users to set the refresh rate for the UI updates.
- **Keyboard Interaction**: Exit the graphical interface by pressing 'q'.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or create issues for bugs, questions, or new features.