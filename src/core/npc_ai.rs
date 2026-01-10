use rand::Rng;

use crate::core::game::GameState;

impl GameState {
    pub fn npc_turns(&mut self) {
        for npc in &self.world.npcs {
            let dx = (npc.pos().x as i32 - self.player.charcter.pos().x as i32).abs();
            let dy = (npc.pos().y i32 - self.player.character.pos().y as i32).abs();

            //NPC attacks if close
            if dx +dy == 1 {
                self.npc_attack_player(npc);
            }
        }
    }
}