use crate::prelude::*;

// this macro transforms a function
// player_input into player_input_system
// write_component requests writable access to a component type
// read_component requests read-only access to a component type
#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    // #[resource] requests access to types you stored in Legion's resource handler.
    // it's also a procedural macro
    #[resource] map: &Map, // this is just like borrowing elsewhere, request read-only reference to the map
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera, // this is just like a mutable borrow, request multable
                                     // reference to the camera, change the content of the camera,
                                     // and the global resource is updated with the new values.
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
