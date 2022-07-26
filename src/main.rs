#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct State {
    player: Player,

    // This tracks the time accumulated between frames to control the game’s speed.
    frame_time: f32,
    mode: GameMode, // enum type
    obstacle: Obstacle,
    score: i32,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();

        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // Draw the top half of the obstacles
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // Draw the bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    // This function receives a borrowed reference to the player as a parameter
    fn hit_obstracle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // set is a ‘bracket-lib‘ function that sets a single character on the screen.
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@')); // converts a Unicode symbol from your source code to the matching Codepage 437 character number
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32; // this is a cast to i32 to make the sum with self.y
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

impl State {
    // Constructor that return self
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // spefify the background color

        // The tick function runs as fast as it can—often 60 or more times per second.
        // The context provides a variable named frame_time_ms containing the time
        // elapsed since the last time tick was called.
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstracle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    // This is the principal functions to welcome
    // the user when the game is started, we set
    // this in the constructor, set a mode, and the
    // tick fn this is matched and applied
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to xGame");
        ctx.print_centered(10, "(P) Play Game");
        ctx.print_centered(12, "(Q) Quit Game");

        self.apply_click(ctx);
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
        self.mode = GameMode::Playing
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));

        ctx.print_centered(10, "(P) Play Again");
        ctx.print_centered(12, "(Q) Quit Game");

        self.apply_click(ctx);
    }

    // &mut self is a reference to `&mut State`
    fn apply_click(&mut self, ctx: &mut BTerm) {
        // Check what type the letter the user choice
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
                   // ctx.print(1, 1, "loading..."); // 1, 1 is the coordiantes
        ctx.print_centered(20, "loading...");
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
