use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// with this derives is possible to call mytile.Clone() etc ..
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

#[allow(dead_code)]
impl Map {
    pub fn new() -> Self {
        let x = TileType::Wall;
        _ = x;

        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    // received a borrowed camera to render just the visible part of the map
    // this is not more used because used instead the Legions render map, into systems module
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // this tells the library to render to the first console layer, the base map
        ctx.set_active_console(0);

        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                // use self.in_bounds to make sure tha each tile exist
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);

                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }
    }

    // this function is take to player only walk into the right away, and dont over
    // walls
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // check if destination is valid and if a tile is a floor
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // determining a tile's index coordinates
    // return None no value or Some(index)
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        // verify map coordinate is valid
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

// return usize because vectors are indexed by
// a variable of type usize
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
