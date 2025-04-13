# Chaikin's Algorithm Animation

An interactive visualization of Chaikin's curve subdivision algorithm implemented in Rust using the minifb window library.

## System Requirements

### Linux Dependencies
To run this application on Linux, you need the following system packages:

```bash
sudo apt-get install -y libxkbcommon-dev libwayland-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev libx11-dev
```

### Rust Dependencies
The project uses the following Rust crates:
- minifb (0.23): Window creation and graphics
- nalgebra (0.33): Vector mathematics
- rand (0.8): Random color generation
- log (0.4): Logging functionality
- env_logger (0.10): Logging implementation

## Building and Running

1. Make sure you have Rust and Cargo installed
2. Install the required system dependencies (see above)
3. Clone the repository
4. Run the application:
```bash
cargo run
```

## Known Issues

- On some Linux systems, you may see a warning message: "Failed to create server-side surface decoration: Missing". This is normal and does not affect the functionality of the application.

## Features

- Interactive curve creation and manipulation
- Real-time curve subdivision using Chaikin's algorithm
- Smooth animation of curve transitions
- Window resizing support
