#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, _: &mut BTerm) {
        self.mode = GameMode::End
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to xGame");
        ctx.print_centered(10, "(P) Play Game");
        ctx.print_centered(12, "(Q) Quit Game");

        // Check what type the letter the user choice
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {} // do nothing
            }
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(10, "(P) Play Again");
        ctx.print_centered(12, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {} // do nothing
            }
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

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
        ctx.print(1, 1, "running..."); // 1, 1 is the coordiantes
                                       // representing where you want the text to appear

        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    // build? only works because we return a BError
    // that's an Result
    let context = BTermBuilder::simple80x50().with_title("xGame").build()?;

    // main_loop starts the game loop and begins calling tick function on every frame
    main_loop(context, State::new())
}
