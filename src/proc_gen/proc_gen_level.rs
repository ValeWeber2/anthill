use rand::{Rng, RngCore, SeedableRng, rngs::StdRng, seq::IndexedRandom};

use crate::{
    proc_gen::{bsp::MapBSPTree, proc_gen_world::ProcGenWorld},
    world::{
        coordinate_system::Point,
        level_data::{LevelData, RoomData, SpawnData, TileData, TileTypeData},
        worldspace::{WORLD_HEIGHT, WORLD_WIDTH},
    },
};

/// Data Structure that holds all data for a level that is being procedurally generated.
/// This data structure is composed of other data structures involved in the procedural generation process.
pub struct ProcGenLevel {
    /// Data structure containing a procedurally generated world.
    pub world: ProcGenWorld,

    /// Contains the entry point, where the player appears upon reaching the level.
    pub entry: Point,

    /// Contains the exit point, where stairs that lead to the next level will be placed.
    pub exit: Point,

    /// Contains the lots of `SpawnData` for the entire world. (Items and Npcs)
    pub spawns: Vec<SpawnData>,
}

impl ProcGenLevel {
    /// Main entry point into the procedural generation script.
    /// Generates a new RNG instance with the given seed. This way the world generation remains deterministic.
    pub fn generate(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let bsp_seed = rng.next_u64();
        let room_shrinking_seed = rng.next_u64();
        let corridor_seed = rng.next_u64();
        let population_seed = rng.next_u64();

        let bsp = MapBSPTree::generate_bsp(bsp_seed);
        let proc_gen_world =
            ProcGenWorld::generate_from_bsp(bsp, room_shrinking_seed, corridor_seed);

        ProcGenLevel::generate_from_world(proc_gen_world, population_seed)
    }

    /// Function to extend a [ProcGenWorld] into a [ProcGenLevel].
    ///
    /// Similar to an implementation of the `From` trait, but it couldn't be used, since this method has the rng instance as a dependency.
    ///
    /// # Usage
    /// Call [ProcGenLevel::generate] with a seed to start the world generation.
    fn generate_from_world(world: ProcGenWorld, population_seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(population_seed);

        let mut level = ProcGenLevel {
            world,
            entry: Point::default(),
            exit: Point::default(),
            spawns: Vec::new(),
        };

        level.populate(&mut rng);
        level.add_entry_exit(&mut rng);

        level
    }

    /// Adds entry points and exit points for the Map (which will be turned into stairs, up and down respectively)
    pub fn add_entry_exit<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        // Define rooms that need to exist on every level.
        let mut mandatory_rooms = self.world.rooms.choose_multiple(rng, 2);
        let entry_room = mandatory_rooms
            .next()
            .expect("Could not choose from rooms because the room number is 0.");
        let exit_room = mandatory_rooms
            .next()
            .expect("Could not choose from rooms because the room number is 0.");

        // Determine entry
        let entry_room_floor = entry_room.floor_points();
        let entry_point = entry_room_floor
            .choose(rng)
            .expect("Room smaller than 0. Rooms are by definition bigger than 0");
        self.entry = *entry_point;

        // Determine exit
        let exit_room_floor = exit_room.floor_points();
        let exit_point = exit_room_floor
            .choose(rng)
            .expect("Room smaller than 0. Rooms are by definition bigger than 0");
        self.exit = *exit_point;
    }
}

impl From<ProcGenLevel> for LevelData {
    fn from(value: ProcGenLevel) -> Self {
        let room_data: Vec<RoomData> = value.world.rooms.into_iter().map(RoomData::from).collect();

        let mut tiles: Vec<TileData> = vec![
            // Entry
            TileData { x: value.entry.x, y: value.entry.y, tile_type: TileTypeData::StairsUp },
            // Exit
            TileData { x: value.exit.x, y: value.exit.y, tile_type: TileTypeData::StairsDown },
        ];

        for (point, door_type) in value.world.corridor_map.doors {
            tiles.push(TileData {
                x: point.x,
                y: point.y,
                tile_type: TileTypeData::Door(door_type),
            });
        }

        LevelData {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            tiles,
            rooms: room_data,
            corridors: value.world.corridor_map.corridors,
            entry: value.entry,
            exit: value.exit,
            spawns: value.spawns,
        }
    }
}
