pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    // push a tuple
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
