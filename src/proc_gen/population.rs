use std::ops::Range;

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
    seq::IndexedRandom,
};

use crate::{
    data::{item_defs::item_defs, npc_defs::npc_defs},
    proc_gen::{bsp::MapBSP, bsp_nodes::MapNode},
    world::{
        coordinate_system::Point,
        world_data::{SpawnData, SpawnKind},
    },
};

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

impl MapBSP {
    pub fn populate_rooms<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        for node_id in self.rooms.clone() {
            let node = self.get_node_mut(node_id);

            let encounter: RoomEncounter = rng.random();

            let mut population = node.populate(encounter, rng);
            self.spawns.append(&mut population);
        }
    }
}

impl MapNode {
    pub fn populate<R: Rng + ?Sized>(
        &mut self,
        encounter: RoomEncounter,
        rng: &mut R,
    ) -> Vec<SpawnData> {
        let available_x = (self.point_a.x + 1)..(self.point_b.x - 1);
        let available_y = (self.point_a.y + 1)..(self.point_b.y - 1);

        match encounter {
            RoomEncounter::Empty => Vec::new(),
            RoomEncounter::Enemy => {
                let mut population = Vec::new();
                population.append(&mut random_npcs(available_x, available_y, rng));
                population
            }
            RoomEncounter::EnemyTreasure => {
                let mut population = Vec::new();
                population.append(&mut random_npcs(available_x.clone(), available_y.clone(), rng));
                population.append(&mut random_items(available_x, available_y, rng));
                population
            }
            RoomEncounter::Treasure => {
                let mut population = Vec::new();
                population.append(&mut random_items(available_x, available_y, rng));
                population
            }
        }
    }
}

fn random_npcs<R: Rng + ?Sized>(
    available_x: Range<usize>,
    available_y: Range<usize>,
    rng: &mut R,
) -> Vec<SpawnData> {
    let spawns_amount = rng.random_range(1..3);

    let mut spawns: Vec<SpawnData> = Vec::new();
    for _ in 0..spawns_amount {
        let npcs: Vec<&String> = npc_defs().keys().collect();
        if let Some(npc) = npcs.choose(rng) {
            let point = Point::new(
                rng.random_range(available_x.clone()),
                rng.random_range(available_y.clone()),
            );

            let spawn_kind = SpawnKind::Npc { def_id: npc.to_string() };
            spawns.push(SpawnData { kind: spawn_kind, x: point.x, y: point.y });
        }
    }

    spawns
}

fn random_items<R: Rng + ?Sized>(
    available_x: Range<usize>,
    available_y: Range<usize>,
    rng: &mut R,
) -> Vec<SpawnData> {
    let spawns_amount = rng.random_range(1..2);

    let mut spawns: Vec<SpawnData> = Vec::new();
    for _ in 0..spawns_amount {
        let items: Vec<&String> = item_defs().keys().collect();
        if let Some(item) = items.choose(rng) {
            let point = Point::new(
                rng.random_range(available_x.clone()),
                rng.random_range(available_y.clone()),
            );

            let spawn_kind = SpawnKind::Item { def_id: item.to_string() };
            spawns.push(SpawnData { kind: spawn_kind, x: point.x, y: point.y });
        }
    }

    spawns
}
