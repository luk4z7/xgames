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

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            // generate Rectangles randomly
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            // verify is a room generated intersect the existance rooms
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
            }

            self.rooms.push(room);
        }
    }

    //
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};

        // iterate from min to max ranges
        // eg: 1..3 equals to 1, 2, 3
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};

        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        // make a deep copy of the variable without affecting the original
        let mut rooms = self.rooms.clone();

        // sort_by require a closure, and send a pair of rooms to the closure
        // this closure receive this as |a, b|
        // compare the coordinates a.center.x with b.center.x with the cmp function
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // iterator has enumerate funcionality, enumerate counts items in the iterator
        // and includes them as the first entry in a tutle
        // skip the first one
        // to create a tutles you can use:
        // let tuple = (a, b, c)
        // access them tuple.0, tuple.1
        // and destructuring
        // let (a, b, c) = tuple;
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            // random between 0 and 2
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
            }
        }
    }
}
