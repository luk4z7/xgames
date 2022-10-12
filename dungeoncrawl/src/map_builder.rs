use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,            // the map it self
    pub rooms: Vec<Rect>,    // number of rooms added to a map
    pub player_start: Point, // store the location of the player
}

impl MapBuilder {
    // exclusive borrow that allows mutability
    fn fill(&mut self, tile: TileType) {
        // iter_mut Returns an iterator that allows modifying each value
        // for_each Calls a closure on each element of an iterator
        // change every tile into a wall, this *t a de-reference
        // de-referenced indicates that you want to write to the referenced variable
        //
        // This illustrates how closure syntax is similar to function syntax except for
        // the use of pipes and the amount of syntax that is optional:
        // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
        // let add_one_v2 = |x: u32| -> u32 { x + 1 };
        // let add_one_v3 = |x|             { x + 1 };
        // let add_one_v4 = |x|               x + 1  ;
        //
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
}
