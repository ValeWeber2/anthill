use rand::Rng;

use crate::{
    proc_gen::{bsp::MapBSPTree, proc_gen_room::ProcGenRoom},
    world::coordinate_system::Point,
};

/// Data Structure that contains the procedurally generated world.
pub struct ProcGenWorld {
    /// Rooms of the map
    pub rooms: Vec<ProcGenRoom>,

    /// Vector of all the tiles that will become hallways on the map.
    pub corridors: Vec<Point>,
}

impl ProcGenWorld {
    /// Function to turn a [MapBSPTree] (Binary Search Partitions) into a [ProcGenWorld].
    ///
    /// Similar to an implementation of the `From` trait, but it couldn't be used, since this method has the rng instance as a dependency.
    pub fn generate_from_bsp<R: Rng + ?Sized>(bsp: MapBSPTree, rng: &mut R) -> Self {
        let rooms = bsp.collect_leaves().into_iter().map(ProcGenRoom::from).collect();

        let mut world = Self { rooms, corridors: Vec::new() };

        world.shrink_rooms(rng);
        world.a_star_corridors(rng);

        world
    }

    /// Shrinks all rooms contained in the [ProcGenWorld]
    pub fn shrink_rooms<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        for room in &mut self.rooms {
            room.shrink(rng);
        }
    }
}
