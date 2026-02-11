use rand::{SeedableRng, rngs::StdRng};

use crate::{
    proc_gen::bsp::MapBSP,
    world::{world_data::WorldData, world_loader::save_world_to_ron},
};

pub fn generate_map(map_seed: u64) -> WorldData {
    let mut rng = StdRng::seed_from_u64(map_seed);

    let mut map = MapBSP::default();
    map.divide(&mut rng);
    map.leaves_to_rooms(map.root);
    map.shrink_leaves(&mut rng);
    map.find_node_connections(&mut rng);
    map.a_star_corridors(&mut rng);
    map.populate_rooms(&mut rng);
    map.add_entry_exit(&mut rng);

    let world_data = WorldData::from(map);
    let _ = save_world_to_ron(&world_data, "assets/worlds/proc_gen.ron");

    world_data
}
