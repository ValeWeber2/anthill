use rand::Rng;

use crate::core::{
    entity_logic::{Npc},
    game::GameState,
    player::PlayerCharacter,
};

impl GameState {
    pub fn player_attack_npc(&mut self, npc_id: u32) {
        let npc_index = match self.world.npc_index.get(&npc_id) {
            Some(i) => *i,
            None => return,
        };

        let npc = &mut self.world.npcs[npc_index];

        //Dodge
        let dodge_roll: u8 = self.rng.gen_range(0..=100);
        if dodge_roll < npc.stats.dodge_chance() {
            self.log.print(format!("{} dodged the attack!", npc.name()));
            return;
        }

        //Damage
        let damage = self.player.character.attack_damage();
        npc.stats.base.take_damage(mitigated);

        self.log.print(format!(
            "Player hits {} for {} damage!",
            npc.name(),
            mitigated
        ));

        if !npc.stats.base.is_alive() {
            self.log.print(format!("{} died!", npc.name()));
            self.despawn(npc_id);
            self.player.character.gain_experience(25);
        }
    }

    pub fn npc_attack_player(&mut self, npc: &Npc) {
        let dodge_roll: u8 = self.rng.gen_range(0..=100);
        if dodge_roll < self.player.character.dodge_chance() {
            self.log.print("Player dodged the attack!".to_string());
            return;
        }


        let damage = npc.stats.damage as u32;
        self.player.character.take_damage(mitigated);

        self.log.print(format!(
            "{} hits Player for {} damage!",
            npc.name(),
            mitigated,
        ));
    }
}