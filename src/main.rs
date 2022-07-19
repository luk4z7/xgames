#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

struct State {}

// implements a trait the brackets expos
// This is similar to implementing functions for a structure,
// but you implement the trait for your struct.
impl GameState for State {
    // &mut self allow the tick functions to change your
    // instance state
    fn tick(&mut self, ctx: &mut BTerm) {
        // accessing information like mouse position and
        // keyboard input, and sending commands to draw to the window.
        ctx.cls(); // cls clear the window
        ctx.print(1, 1, "Hello, Bracket Terminal!"); // 1, 1 is the coordiantes
                                                     // representing where you want the text to appear
    }
}

fn main() -> BError {
    // build? only works because we return a BError
    // that's an Result
    let context = BTermBuilder::simple80x50().with_title("X-Dragon").build()?;
    main_loop(context, State {})
}
