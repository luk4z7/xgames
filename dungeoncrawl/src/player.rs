use crate::prelude::*;

pub struct Player {
    pub position: Point, // Point is exported by bracket-lib
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    // receive the position and set into a screen
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    // make a movements abound the screen when move the keyboard keys
    // receives multable camera, it will use it to send up-dates if the player moves
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        // check what key is pressed and move the player into a screen
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            // calculate the player position
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}
