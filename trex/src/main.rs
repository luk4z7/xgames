#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 400;

struct Dood {
    x: i32,
    y: i32,
}

struct State {
    frame: usize,
    timer: f32,
    doods: Vec<Dood>,
    obstacle: Obstacle,
    //    rng: RandomNumberGenerator,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();

        Obstacle {
            x,
            gap_y: random.range(5, 20),
            size: i32::max(2, 10 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        for x in 0..SCREEN_WIDTH {
            ctx.set(x, SCREEN_HEIGHT - 1, WHITE, WHITE, to_cp437('#'));
        }

        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, WHITE, NAVY, 10);
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT - 1 {
            ctx.set(screen_x, y, WHITE, NAVY, 10);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        ctx.set_active_console(1);
        ctx.cls();
        ctx.print(1, 1, "Watch them go!");
        ctx.printer(
            1,
            2,
            &format!("#[pink]FPS: #[]{}", ctx.fps),
            TextAlign::Left,
            None,
        );

        ctx.set_active_console(0);
        ctx.cls();

        let mut random = RandomNumberGenerator::new();
        self.obstacle.render(ctx, random.range(5, 20));

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

            //        for dood in self.doods.iter_mut() {
            //               dood.x += self.rng.range(0, 2) - 1;
            //             dood.y += self.rng.range(0, 2) - 1;
            //      }
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
            //        .add_sprite(Rect::with_size(170, 0, 85, 132))
            //           .add_sprite(Rect::with_size(255, 0, 85, 132)),
        )
        //      .with_sprite_sheet(
        //        SpriteSheet::new("dinod.png")
        //          .add_sprite(Rect::with_size(1200, 1200, 90, 200))
        //        .add_sprite(Rect::with_size(85, 0, 85, 132)), // .add_sprite(Rect::with_size(170, 0, 85, 132))
        // .add_sprite(Rect::with_size(255, 0, 85, 132)),
        //)
        .with_vsync(false)
        .build()?;

    //    let mut rng = RandomNumberGenerator::new();
    let mut doods = Vec::new();
    // for _ in 0..100 {
    doods.push(Dood {
        //       x: rng.range(0, 100),
        //     y: rng.range(0, 100),
        x: 90,
        y: 220,
    });
    //}

    let gs = State {
        frame: 10,
        timer: 0.0,
        doods,
        obstacle: Obstacle::new(SCREEN_WIDTH, 80),
        //      rng,
    };

    main_loop(context, gs)
}
