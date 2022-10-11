#![warn(clippy::all, clippy::pedantic)]

// import the module map to a global scope
// add the module to a project with "mod"
mod map;
use prelude::*;

// prelude to evicted to import everything every time, less verbose
// declare a new module
// this don't need the pub because this is the top of crate, so this is
// visible throughout our program, we can use in another files use crate::prelude::*;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
