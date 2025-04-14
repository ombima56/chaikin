# Chaikin's Algorithm Animation

An interactive visualization of Chaikin's curve subdivision algorithm implemented in Rust using the minifb window library.

## System Requirements
* You need to have Rust installed in your machine.

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

3. Clone the repository and navigate to the directory.

```bash
$ git clone https://learn.zone01kisumu.ke/git/hiombima/chaikin.git
$ cd chaikin
```
4. Run the application:
```bash
$ cargo run
```
5. To run the unit tests:
```bash
$ cargo test
```

## Usage

* To be able to see the animation you need to first add control points. You can do this by left-clicking the mouse on the canvas displayed when you run the program.

* You can add multiple points. Once you are done click ENTER button on your keyboard to start the animation. 

* You should add at least 2 points to get the animation. Two points will produce a straight line. To get a curve you'll have to add multiple points e.g 3 points at different angles creating an arrow-shaped path.

* To clear the canvas click the space bar.


## Known Issues

- On some Linux systems, you may see a warning message: "Failed to create server-side surface decoration: Missing". This is normal and does not affect the functionality of the application.

## Features

- Interactive curve creation and manipulation
- Real-time curve subdivision using Chaikin's algorithm
- Smooth animation of curve transitions
- Window resizing support
