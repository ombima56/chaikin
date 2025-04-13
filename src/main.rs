mod canvas;
mod chaikin;
mod input;
mod point;

use canvas::Canvas;
use input::InputHandler;

fn main() {
    // Initialize logging
    env_logger::init();
    
    // Create canvas and input handler
    let mut canvas = Canvas::new(800, 600);
    let mut input = InputHandler::new();
    
    // Main loop
    while canvas.is_open() && !input.should_close() {
        if let Err(e) = canvas.update(&mut input) {
            eprintln!("Error in main loop: {}", e);
            break;
        }
    }
}
