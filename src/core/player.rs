#![allow(dead_code)]

use std::collections::HashMap;
use std::time::Instant;

use crate::core::entity_logic::{BaseStats, Entity, EntityBase, EntityId, Movable};
use crate::core::game_items::{ArmorItem, GameItemId, PotionEffect, WeaponItem};
use crate::world::coordinate_system::Point;
use ratatui::style::Color;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]

pub enum PotionType {
    Heal,
    Strength,
    Dexterity,
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

pub struct Player {
    pub name: String,
    pub character: PlayerCharacter,
}

impl Player {
    pub fn new(id: EntityId) -> Self {
        Self { name: "Hero".to_string(), character: PlayerCharacter::new(id) }
    }
}

impl Default for Player {
    // for testing, don't insert default player into the world!
    fn default() -> Self {
        Self { name: "Hero".to_string(), character: PlayerCharacter::default() }
    }
}

pub struct PlayerCharacter {
    pub base: EntityBase,
    pub stats: PcStats,
    pub inventory: Vec<GameItemId>,
    pub armor: Option<ArmorItem>,
    pub weapon: Option<WeaponItem>,
    pub active_buffs: Vec<ActiveBuff>,
    pub potion_usage: HashMap<PotionType, PotionUsage>,
}

impl PlayerCharacter {
    pub fn new(id: EntityId) -> Self {
        Self {
            base: EntityBase {
                id,
                name: "Hero".to_string(),
                pos: Point::new(0, 0),
                glyph: '@',
                style: Color::Yellow.into(),
            },
            stats: PcStats::new(),
            inventory: Vec::new(),
            armor: None,
            weapon: None,
            active_buffs: Vec::new(),
            potion_usage: HashMap::new(),
        }
    }
    pub fn attack_damage_bonus(&self) -> u32 {
        let mut bonus = self.stats.strength as u32;

        for buff in &self.active_buffs {
            match buff.effect {
                PotionEffect::Strength { amount, .. } => {
                    bonus += amount;
                }
                PotionEffect::Fatigue { strength_penalty, .. } => {
                    bonus = bonus.saturating_sub(strength_penalty);
                }
                _ => {}
            }
        }
        bonus
    }

    pub fn dodge_chance(&self) -> u8 {
        let mut dodge = (self.stats.dexterity / 2).min(50);

        for buff in &self.active_buffs {
            match buff.effect {
                PotionEffect::Dexterity { amount, .. } => {
                    dodge = ((dodge as u32 + amount) as u8).min(100);
                }
                PotionEffect::Cramp { .. } => dodge /= 2,
                _ => {}
            }
        }
        dodge
    }

    pub fn apply_potion_effect(&mut self, potion_type: PotionType, effect: PotionEffect) {
        let (usage_count, elapsed) = {
            let usage = self
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
                self.stats.base.hp_current =
                    (self.stats.base.hp_current + amount).min(self.stats.base.hp_max);
                println!("You regain {} HP.", amount);

                if usage_count >= 3 && elapsed < 30 {
                    let poison_damage = match usage_count {
                        3 => 20,
                        4 => 35,
                        5 => 45,
                        _ => 0,
                    };
                    let tick_damage = poison_damage / 10;
                    self.active_buffs.push(ActiveBuff {
                        effect: PotionEffect::Poison {
                            damage_per_tick: tick_damage,
                            ticks_left: 10,
                        },
                        remaining_turns: 10,
                    });

                    println!("Poisoned! You will take {} HP damage over time.", poison_damage);
                }
            }

            PotionEffect::Strength { amount, duration } => {
                if usage_count < 3 {
                    self.active_buffs.push(ActiveBuff { effect, remaining_turns: duration });
                    println!("Strength increased by {} for {} turns.", amount, duration);
                } else {
                    let strength_penalty: u32 = amount / 2;
                    self.active_buffs.push(ActiveBuff {
                        effect: PotionEffect::Fatigue { strength_penalty, duration },
                        remaining_turns: duration,
                    });
                    println!(
                        "Fatigued! Strength reduced by {} for {} turns.",
                        strength_penalty, duration
                    );

                    if usage_count >= 4 {
                        let hp_loss = 10 / 5;
                        self.active_buffs.push(ActiveBuff {
                            effect: PotionEffect::Poison {
                                damage_per_tick: hp_loss,
                                ticks_left: 5,
                            },
                            remaining_turns: 5,
                        });
                        println!("Overworked! You will take 10 HP damage over 5 turns.");
                    }
                }
            }

            PotionEffect::Dexterity { amount, duration } => {
                if usage_count < 3 {
                    let dex_buff = ActiveBuff {
                        effect: PotionEffect::Dexterity { amount, duration },
                        remaining_turns: duration,
                    };
                    self.active_buffs.push(dex_buff);
                    println!("Dexterity increased by {} for {} turns.", amount, duration);
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

                self.active_buffs.push(cramp_buff);

                if hp_loss > 0 {
                    let poison_buff = ActiveBuff {
                        effect: PotionEffect::Poison {
                            damage_per_tick: hp_loss / penalty_turns as u32,
                            ticks_left: penalty_turns,
                        },
                        remaining_turns: penalty_turns,
                    };
                    self.active_buffs.push(poison_buff);
                }

                println!("Overdose! Movement reduced for {} turns.", penalty_turns);
            }
            _ => {}
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.stats.base.take_damage(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.stats.base.heal(amount);
    }

    pub fn is_alive(&self) -> bool {
        self.stats.base.is_alive()
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.stats.experience += amount;

        let required_xp = self.stats.level as u32 * 100;
        if self.stats.experience >= required_xp {
            self.stats.experience -= required_xp;
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.stats.level += 1;
        self.stats.strength += 1;
        self.stats.dexterity += 1;
        self.stats.vitality += 1;

        self.stats.base.hp_max += 10;
        self.stats.base.hp_current = self.stats.base.hp_max;
    }

    pub fn tick_buffs(&mut self) {
        let mut damage_accrued: u32 = 0;
        for buff in &mut self.active_buffs {
            if let PotionEffect::Poison { ticks_left, damage_per_tick } = &mut buff.effect {
                if *ticks_left > 0 {
                    damage_accrued += *damage_per_tick;
                    *ticks_left -= 1
                }
            }
            if buff.remaining_turns > 0 {
                buff.remaining_turns -= 1;
            }
        }
        self.take_damage(damage_accrued);
        self.active_buffs.retain(|buff| buff.remaining_turns > 0);
    }
}

impl Default for PlayerCharacter {
    fn default() -> Self {
        Self::new(999999) // placeholder, never inserted inro world
    }
}

pub struct PcStats {
    pub base: BaseStats,
    pub strength: u8,
    pub dexterity: u8,
    pub vitality: u8,
    pub perception: u8,
    pub level: u8,
    pub experience: u32,
}

impl PcStats {
    pub fn new() -> Self {
        let vitality = 10;
        let hp_max = vitality as u32 * 10;

        Self {
            base: BaseStats { hp_max, hp_current: hp_max },
            strength: 10,
            dexterity: 10,
            vitality,
            perception: 10,
            level: 1,
            experience: 0,
        }
    }
}

impl BaseStats {
    pub fn take_damage(&mut self, amount: u32) {
        if amount >= self.hp_current {
            self.hp_current = 0;
        } else {
            self.hp_current -= amount;
        }
    }

    pub fn heal(&mut self, amount: u32) {
        self.hp_current = (self.hp_current + amount).min(self.hp_max);
    }

    pub fn is_alive(&self) -> bool {
        self.hp_current > 0
    }
}

impl Entity for PlayerCharacter {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn id(&self) -> EntityId {
        self.base.id
    }
    fn pos(&self) -> &Point {
        &self.base.pos
    }
}

impl Movable for PlayerCharacter {
    fn move_to(&mut self, point: Point) {
        self.base.pos.x = point.x;
        self.base.pos.y = point.y;
    }
}

#[derive(Clone)]
pub struct Weapon {
    pub base_damage: u32,
    pub crit_chance: u8,
}

impl Weapon {
    pub fn new(base_damage: u32, crit_chance: u8) -> Self {
        Self { base_damage, crit_chance }
    }
}
