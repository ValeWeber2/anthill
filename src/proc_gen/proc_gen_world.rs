use rand::{SeedableRng, rngs::StdRng};

use crate::proc_gen::{bsp::MapBSPTree, corridors::ProcGenCorridorMap, proc_gen_room::ProcGenRoom};

/// Data Structure that contains the procedurally generated world.
pub struct ProcGenWorld {
    /// Rooms of the map
    pub rooms: Vec<ProcGenRoom>,

    /// Data containing corridor tiles and door data.
    pub corridor_map: ProcGenCorridorMap,
}

impl ProcGenWorld {
    /// Function to turn a [MapBSPTree] (Binary Search Partitions) into a [ProcGenWorld].
    ///
    /// Similar to an implementation of the `From` trait, but it couldn't be used, since this method has the rng instance as a dependency.
    pub fn generate_from_bsp(
        bsp: MapBSPTree,
        room_shrinking_seed: u64,
        corridor_seed: u64,
    ) -> Self {
        let rooms = bsp.collect_leaves().into_iter().map(ProcGenRoom::from).collect();

        let mut world = Self { rooms, corridor_map: ProcGenCorridorMap::default() };

        world.shrink_rooms(room_shrinking_seed);
        world.corridor_map = world.a_star_corridors(corridor_seed);

        world
    }

    /// Shrinks all rooms contained in the [ProcGenWorld]
    pub fn shrink_rooms(&mut self, room_shrinking_seed: u64) {
        let mut rng = StdRng::seed_from_u64(room_shrinking_seed);

        for room in &mut self.rooms {
            room.shrink(&mut rng);
        }
    }
}
