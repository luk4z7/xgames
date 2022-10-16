#![warn(clippy::all, clippy::pedantic)]

// import the module map to a global scope
// add the module to a project with "mod"
mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
// dont use anymore because of use it of ECS legion
// mod player;
use prelude::*;

// prelude to evicted to import everything every time, less verbose
// declare a new module
// this don't need the pub because this is the top of crate, so this is
// visible throughout our program, we can use in another files use crate::prelude::*;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    // pub use crate::player::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
    // old model using without ecs
    // map: Map,
    // player: Player,
    // camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        let mut ecs = World::default();
        let mut resources = Resources::default();
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        spawn_player(&mut ecs, map_builder.player_start);

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
            // old example
            // map: Map::new(),
            // player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
            // map: map_builder.map,
            // player: Player::new(map_builder.player_start),
            // camera: Camera::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");

        // self.player.update(ctx, &self.map, &mut self.camera);
        // self.map.render(ctx, &self.camera);
        // self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        // with_dimensions specifies the size of subsequent consoles you add
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        // 32x32 is the dimension of the character
        .with_tile_dimensions(32, 32)
        // The directory in which you placed the graphics file.
        .with_resource_path("resources/")
        // fonts is the same of .with_tile_dimensions in most cases
        .with_font("dungeonfont.png", 32, 32)
        // Add a console using the dimensions already specified and the named tile graphics file.
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // Add a second console with no background so transparency shows through it
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
