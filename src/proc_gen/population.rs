use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
    seq::{IndexedRandom, SliceRandom},
};

use crate::{
    data::{item_defs::item_defs, npc_defs::npc_defs},
    proc_gen::{proc_gen_level::ProcGenLevel, proc_gen_room::ProcGenRoom},
    world::{
        coordinate_system::Point,
        level_data::{SpawnData, SpawnKind},
    },
};

/// Defines all possible "Encounters", which are variants for how a room can be populated.
///
/// This implements [Distribution], where the chances of each random `RoomEncounter` are defined
pub enum RoomEncounter {
    Empty,
    Enemy,
    EnemyTreasure,
    Treasure,
    // Trap
}

impl Distribution<RoomEncounter> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RoomEncounter {
        match rng.random_range(0..100) {
            0..=29 => RoomEncounter::Enemy,
            30..=49 => RoomEncounter::EnemyTreasure,
            50..=74 => RoomEncounter::Treasure,
            _ => RoomEncounter::Empty,
        }
    }
}

impl ProcGenLevel {
    /// Populates the level with npcs.
    ///
    /// Populating a room requires its data, which is why populate is a method on room as well.
    pub fn populate<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let blocked_points: Vec<Point> = vec![self.entry, self.exit];
        for room in &mut self.world.rooms {
            let encounter: RoomEncounter = rng.random();

            let mut population = room.populate(encounter, &blocked_points, rng);
            self.spawns.append(&mut population);
        }
    }
}

impl ProcGenRoom {
    /// Populates the room with spawn points for NPCs and Data
    ///
    /// # Arguments
    /// * `encounter`: Type of encounter. Defines what should be spawned.
    /// * `blocked_points`: Points that cannot be spawn points.
    /// * `rng`: Rng Instance.
    pub fn populate<R: Rng + ?Sized>(
        &mut self,
        encounter: RoomEncounter,
        blocked_points: &[Point],
        rng: &mut R,
    ) -> Vec<SpawnData> {
        let mut available_points = self.floor_points();
        available_points.retain(|point| !blocked_points.contains(point));
        available_points.shuffle(rng);

        let mut population = Vec::new();

        match encounter {
            RoomEncounter::Empty => {}
            RoomEncounter::Enemy => {
                population.append(&mut random_npcs(&mut available_points, rng));
            }
            RoomEncounter::EnemyTreasure => {
                population.append(&mut random_npcs(&mut available_points, rng));
                population.append(&mut random_items(&mut available_points, rng));
            }
            RoomEncounter::Treasure => {
                population.append(&mut random_items(&mut available_points, rng));
            }
        }

        population
    }
}

/// Helper method that randomly selects npcs to spawn and where to put them.
fn random_npcs<R: Rng + ?Sized>(available_points: &mut Vec<Point>, rng: &mut R) -> Vec<SpawnData> {
    let spawns_amount = rng.random_range(1..3);

    let mut spawns: Vec<SpawnData> = Vec::new();
    for _ in 0..spawns_amount {
        let mut npcs: Vec<&String> = npc_defs().keys().collect();
        npcs.sort(); // The definitions need to be sorted because apparently HashMaps are random.

        if let Some(npc_def_id) = npcs.choose(rng) {
            if let Some(point) = available_points.pop() {
                let spawn_kind = SpawnKind::Npc { def_id: npc_def_id.to_string() };
                spawns.push(SpawnData { kind: spawn_kind, x: point.x, y: point.y });
            }
        }
    }

    spawns
}

/// Helper method that randomly selects items to spawn as sprites and where to put them.
fn random_items<R: Rng + ?Sized>(available_points: &mut Vec<Point>, rng: &mut R) -> Vec<SpawnData> {
    let spawns_amount = rng.random_range(1..2);

    let mut spawns: Vec<SpawnData> = Vec::new();
    for _ in 0..spawns_amount {
        let mut item_defs: Vec<&String> = item_defs().keys().collect();
        item_defs.sort(); // The definitions need to be sorted because apparently HashMaps are random.

        if let Some(item_def_id) = item_defs.choose(rng) {
            if let Some(point) = available_points.pop() {
                let spawn_kind = SpawnKind::Item { def_id: item_def_id.to_string() };
                spawns.push(SpawnData { kind: spawn_kind, x: point.x, y: point.y });
            }
        }
    }

    spawns
}
