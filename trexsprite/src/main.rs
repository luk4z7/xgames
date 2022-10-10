#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

struct Dood {
    x: i32,
    y: i32,
}

struct State {
    frame: usize,
    timer: f32,
    doods: Vec<Dood>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        for dood in self.doods.iter() {
            ctx.add_sprite(
                Rect::with_size(dood.x, dood.y, 80, 100),
                0 - dood.y,
                RGBA::from_f32(4.0, 5.0, 5.0, 5.0),
                self.frame % 2,
            )
        }

        self.timer += ctx.frame_time_ms;
        if self.timer > 66.0 {
            self.timer = 0.0;
            self.frame += 1;
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_sprite_console(640, 400, 0)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console_no_bg(80, 50, "terminal8x8.png")
        .with_title("Bracket Terminal - Sprite Console")
        .with_sprite_sheet(
            SpriteSheet::new("dinod.png")
                .add_sprite(Rect::with_size(936, 45, 44, 60))
                .add_sprite(Rect::with_size(980, 45, 44, 60)),
        )
        .build()?;

    let mut doods = Vec::new();
    doods.push(Dood { x: 90, y: 220 });

    let gs = State {
        frame: 0,
        timer: 0.0,
        doods,
    };

    main_loop(context, gs)
}
