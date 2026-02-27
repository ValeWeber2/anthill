use crate::{core::game::GameState, util::text_log::LogData};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PotionType {
    Heal,
    Strength,
    Dexterity,
    Poison,
    Fatigue,
    Cramp,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PotionEffectDef {
    Heal { amount: u16 },
    Strength { amount: u8, duration: u8 },
    Dexterity { amount: u8, duration: u8 },
    Poison { damage_per_tick: u16, duration: u8 },
    Fatigue { strength_penalty: u8, duration: u8 },
    Cramp { dexterity_penalty: u8, duration: u8 },
}

#[derive(Clone, Debug)]
pub struct PotionUsage {
    pub count: u8,
    pub last_used: u64,
}

#[derive(Clone, Debug)]
pub struct ActiveBuff {
    pub effect: PotionEffectDef,
    pub remaining_turns: u8,
}

impl GameState {
    pub fn apply_potion_effect(&mut self, effect: PotionEffectDef) {
        let potion_type: PotionType = match effect {
            PotionEffectDef::Heal { .. } => PotionType::Heal,
            PotionEffectDef::Strength { .. } => PotionType::Strength,
            PotionEffectDef::Dexterity { .. } => PotionType::Dexterity,
            PotionEffectDef::Poison { .. } => PotionType::Poison,
            PotionEffectDef::Fatigue { .. } => PotionType::Fatigue,
            PotionEffectDef::Cramp { .. } => PotionType::Cramp,
        };

        let (usage_count, rounds_since_last_use) = {
            let usage = self
                .player
                .character
                .potion_usage
                .entry(potion_type)
                .or_insert(PotionUsage { count: 0, last_used: self.round_nr });

            let rounds_since_last_use = self.round_nr - usage.last_used;
            usage.count += 1;
            usage.last_used = self.round_nr;
            (usage.count, rounds_since_last_use)
        };

        match effect {
            PotionEffectDef::Heal { amount } => {
                self.player.character.heal(amount);
                self.log.info(LogData::PlayerHealed { amount });

                if usage_count >= 3 && rounds_since_last_use < 30 {
                    self.player.character.active_buffs.push(ActiveBuff {
                        effect: PotionEffectDef::Poison { damage_per_tick: 2, duration: 10 },
                        remaining_turns: 10,
                    });

                    self.log.info(LogData::Overdose);
                }
            }
            PotionEffectDef::Strength { amount, duration } => {
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
                        effect: PotionEffectDef::Fatigue { strength_penalty, duration },
                        remaining_turns: duration,
                    });
                    self.log.print(format!(
                        "Fatigued! Strength reduced by {} for {} turns.",
                        strength_penalty, duration
                    ));

                    if usage_count >= 4 {
                        self.player.character.active_buffs.push(ActiveBuff {
                            effect: PotionEffectDef::Poison { damage_per_tick: 2, duration: 5 },
                            remaining_turns: 5,
                        });
                        self.log.info(LogData::Overdose);
                    }
                }
            }
            PotionEffectDef::Dexterity { amount, duration } => {
                if usage_count < 3 {
                    self.player
                        .character
                        .active_buffs
                        .push(ActiveBuff { effect, remaining_turns: duration });
                    self.log.print(format!(
                        "Dexterity increased by {} for {} turns.",
                        amount, duration
                    ));
                } else {
                    let dexterity_penalty: u8 = amount / 2;
                    self.player.character.active_buffs.push(ActiveBuff {
                        effect: PotionEffectDef::Cramp { dexterity_penalty, duration },
                        remaining_turns: duration,
                    });
                    self.log.print(format!(
                        "You have cramps! Dexterity reduced by {} for {} turns.",
                        dexterity_penalty, duration
                    ));

                    if usage_count >= 4 {
                        self.player.character.active_buffs.push(ActiveBuff {
                            effect: PotionEffectDef::Poison { damage_per_tick: 2, duration: 5 },
                            remaining_turns: 5,
                        });
                        self.log.info(LogData::Overdose);
                    }
                }
            }
            PotionEffectDef::Poison { damage_per_tick: _, duration } => self
                .player
                .character
                .active_buffs
                .push(ActiveBuff { effect, remaining_turns: duration }),
            PotionEffectDef::Fatigue { strength_penalty: _, duration } => self
                .player
                .character
                .active_buffs
                .push(ActiveBuff { effect, remaining_turns: duration }),
            PotionEffectDef::Cramp { dexterity_penalty: _, duration } => self
                .player
                .character
                .active_buffs
                .push(ActiveBuff { effect, remaining_turns: duration }),
        }
    }
}
