#![allow(dead_code)]

use std::time::Instant;

use crate::core::game::GameState;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PotionType {
    Heal,
    Strength,
    Dexterity,
}

#[derive(Clone, Debug)]
pub enum PotionEffect {
    Heal { amount: u16 },
    Strength { amount: u8, duration: u8 },
    Dexterity { amount: u8, duration: u8 },
    Poison { damage_per_tick: u16, ticks_left: u8 },
    Fatigue { strength_penalty: u8, duration: u8 },
    Cramp { dexterity_penalty: u8, duration: u8 },
}

#[derive(Clone, Debug)]
pub struct PotionUsage {
    pub count: u8,
    pub last_used: Instant,
}

#[derive(Clone, Debug)]
pub struct ActiveBuff {
    pub effect: PotionEffect,
    pub remaining_turns: u8,
}

impl GameState {
    pub fn apply_potion_effect(&mut self, potion_type: PotionType, effect: PotionEffect) {
        let (usage_count, elapsed) = {
            let usage = self
                .player
                .character
                .potion_usage
                .entry(potion_type)
                .or_insert(PotionUsage { count: 0, last_used: Instant::now() });

            if usage.count >= 5 {
                //("You cannot carry more of this potion type.");
                return;
            }

            let elapsed = usage.last_used.elapsed().as_secs();
            usage.count += 1;
            usage.last_used = Instant::now();
            (usage.count, elapsed)
        };

        match effect {
            PotionEffect::Heal { amount } => {
                self.player.character.stats.base.hp_current =
                    (self.player.character.stats.base.hp_current + amount)
                        .min(self.player.character.stats.base.hp_max);
                self.log.print(format!("You regain {} HP.", amount));

                if usage_count >= 3 && elapsed < 30 {
                    let poison_damage = match usage_count {
                        3 => 20,
                        4 => 35,
                        5 => 45,
                        _ => 0,
                    };
                    let tick_damage = poison_damage / 10;
                    self.player.character.active_buffs.push(ActiveBuff {
                        effect: PotionEffect::Poison {
                            damage_per_tick: tick_damage,
                            ticks_left: 10,
                        },
                        remaining_turns: 10,
                    });

                    self.log.print(format!(
                        "Poisoned! You will take {} HP damage over time.",
                        poison_damage
                    ));
                }
            }

            PotionEffect::Strength { amount, duration } => {
                if usage_count < 3 {
                    self.player
                        .character
                        .active_buffs
                        .push(ActiveBuff { effect, remaining_turns: duration });
                    self.log
                        .print(format!("Strength increased by {} for {} turns.", amount, duration));
                } else {
                    let strength_penalty: u8 = amount / 2;
                    self.player.character.active_buffs.push(ActiveBuff {
                        effect: PotionEffect::Fatigue { strength_penalty, duration },
                        remaining_turns: duration,
                    });
                    self.log.print(format!(
                        "Fatigued! Strength reduced by {} for {} turns.",
                        strength_penalty, duration
                    ));

                    if usage_count >= 4 {
                        let hp_loss = 10 / 5;
                        self.player.character.active_buffs.push(ActiveBuff {
                            effect: PotionEffect::Poison {
                                damage_per_tick: hp_loss,
                                ticks_left: 5,
                            },
                            remaining_turns: 5,
                        });
                        self.log.print(
                            "Overworked! You will take 10 HP damage over 5 turns.".to_string(),
                        );
                    }
                }
            }

            PotionEffect::Dexterity { amount, duration } => {
                if usage_count < 3 {
                    let dex_buff = ActiveBuff {
                        effect: PotionEffect::Dexterity { amount, duration },
                        remaining_turns: duration,
                    };
                    self.player.character.active_buffs.push(dex_buff);
                    self.log.print(format!(
                        "Dexterity increased by {} for {} turns.",
                        amount, duration
                    ));
                    return;
                }

                let (penalty_turns, hp_loss) = match usage_count {
                    3 => (2, 0),
                    4 => (1, 0),
                    5 => (3, 10),
                    _ => return,
                };

                let cramp_buff = ActiveBuff {
                    effect: PotionEffect::Cramp {
                        dexterity_penalty: amount,
                        duration: penalty_turns,
                    },
                    remaining_turns: penalty_turns,
                };

                self.player.character.active_buffs.push(cramp_buff);

                if hp_loss > 0 {
                    let poison_buff = ActiveBuff {
                        effect: PotionEffect::Poison {
                            damage_per_tick: hp_loss / penalty_turns as u16,
                            ticks_left: penalty_turns,
                        },
                        remaining_turns: penalty_turns,
                    };
                    self.player.character.active_buffs.push(poison_buff);
                }

                self.log.print(format!("Overdose! Movement reduced for {} turns.", penalty_turns));
            }
            _ => {}
        }
    }
}
