#![allow(dead_code)]

use rand::Rng;

use crate::{core::game::GameState, world::coordinate_system::Direction};

/// DieSize represents the size of a die, meaning how many sides the die has.
#[derive(Clone, Copy, Debug)]
pub enum DieSize {
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D20 = 20,
    D100 = 100,
}

impl DieSize {
    fn upper_bound(self) -> u8 {
        self as u8
    }
}

/// RNG Dice Rolls
///
/// The roll is resolved by calling [`Roll::roll`] and injecting a mutale reference to an RNG.
///
/// # Example
/// ```
/// use rand::{SeedableRng, rngs::StdRng};
///
/// let mut rng = StdRng::seed_from_u64(73);
///
/// let strength: i16 = 5;
/// let penalty: i16 = -2;
///
/// let result = Roll::new(1, DieSize::D6)
///     .add_modifier(strength)
///     .add_modifier(penalty)
///     .roll(&mut rng);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Roll {
    /// Number of dice to be rolled.
    dice_amount: u8,
    /// Die size (number of sides) of the dice.
    dice_size: DieSize,
    /// Modifier to be applied to the result.
    modifier: i16,
}

impl Roll {
    pub fn new(dice_amount: u8, dice_size: DieSize) -> Self {
        Self { dice_amount, modifier: i16::default(), dice_size }
    }

    pub fn add_modifier(mut self, modifier: i16) -> Self {
        self.modifier += modifier;
        self
    }

    pub fn roll<R: Rng + ?Sized>(&self, rng: &mut R) -> i16 {
        let roll = rng.random_range(1..=self.dice_size.upper_bound()) as i16;
        roll + self.modifier
    }
}

/// RNG Resolution using [`Roll`]s.
///
/// A `Check` succeeds if the resoled roll result is >= the configured difficulty
///
/// # Example
/// ```
/// use rand::{SeedableRng, rngs::StdRng};
///
/// let mut rng = StdRng::seed_from_u64(73);
///
/// let strength: i16 = 5;
/// let penalty: i16 = -2;
/// let difficulty: i16 = 10;
///
/// let success = Check::default()
///     .add_modifier(strength)
///     .add_modifier(penalty)
///     .set_difficulty(difficulty)
///     .resolve(&mut rng);
/// ```///
pub struct Check {
    roll: Roll,
    /// Target number that must be met for a success.
    difficulty: i16,
}

impl Check {
    pub fn new(roll: Roll) -> Self {
        Self { roll, difficulty: i16::default() }
    }

    pub fn add_modifier(self, modifier: i16) -> Self {
        self.roll.add_modifier(modifier);
        self
    }

    pub fn set_difficulty(mut self, difficulty: i16) -> Self {
        self.difficulty = difficulty;
        self
    }

    pub fn resolve<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        let rolled_num = self.roll.roll(rng);
        rolled_num >= self.difficulty
    }
}

impl Default for Check {
    /// Creates a standard d20 roll.
    /// This is meant for common checks and attacks.
    fn default() -> Self {
        Self {
            roll: Roll { dice_amount: 1, dice_size: DieSize::D20, modifier: i16::default() },
            difficulty: i16::default(),
        }
    }
}

impl GameState {
    /// Rolls dice using the `GameState`'s internal RNG.
    ///
    /// # Example
    /// ```
    /// let mut game = GameState::new();
    ///
    /// let strength = 5;
    /// let penalty = -2;
    ///
    /// let result = game.roll(
    ///     Roll::new(1, DieSize::D6)
    ///         .add_modifier(strength)
    ///         .add_modifier(penalty),
    /// );
    /// ```
    pub fn roll(&mut self, roll: &Roll) -> i16 {
        roll.roll(&mut self.rng)
    }

    /// Resolves a `Check` using the `GameState`'s internal RNG.
    ///
    /// Usage:
    /// ```
    /// let game = GameState::new();
    ///
    /// let strength = 5;
    /// let penalty = -2;
    /// let difficulty = 15;
    ///
    /// let result: bool = game.check(Check::default().add_modifier(strength).add_modifier(penalty).set_difficulty(difficulty));
    /// ```
    pub fn check(&mut self, check: &Check) -> bool {
        check.resolve(&mut self.rng)
    }
}

impl Direction {
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.random_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            _ => Direction::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn unmodified_roll_in_bounds() {
        let mut rng = StdRng::seed_from_u64(73);

        let roll = Roll::new(1, DieSize::D6);
        let result = roll.roll(&mut rng);

        assert!((1..=6).contains(&result));
    }

    #[test]
    fn modified_roll_modifies_result() {
        let mut rng = StdRng::seed_from_u64(73);

        let base_roll = Roll::new(1, DieSize::D6);
        let modified_roll_positive = base_roll.add_modifier(10);
        let modified_roll_negative = base_roll.add_modifier(-10);

        let base = base_roll.roll(&mut rng);
        let modified_positive = modified_roll_positive.roll(&mut rng);
        let modified_negative = modified_roll_negative.roll(&mut rng);

        assert_eq!(modified_positive, base + 10);
        assert_eq!(modified_negative, base - 10);
    }

    #[test]
    fn stacking_modifiers() {
        let mut rng1 = StdRng::seed_from_u64(73);

        let modified = Roll::new(1, DieSize::D20).add_modifier(5).add_modifier(-2).roll(&mut rng1);

        let mut rng2 = StdRng::seed_from_u64(73);
        let unmodified = Roll::new(1, DieSize::D20).roll(&mut rng2);

        assert_eq!(modified, unmodified + 3);
    }

    #[test]
    fn check_degrees_of_success() {
        let mut rng1 = StdRng::seed_from_u64(73);

        let roll = Roll::new(1, DieSize::D20);
        let value = roll.roll(&mut rng1);

        let check_success = Check::new(roll).set_difficulty(value);
        let check_failure = Check::new(roll).set_difficulty(value + 1);

        let mut rng2 = StdRng::seed_from_u64(73);
        let mut rng3 = StdRng::seed_from_u64(73);

        assert!(check_success.resolve(&mut rng2));
        assert!(!check_failure.resolve(&mut rng3));
    }

    #[test]
    fn modifier_exceeding_difficulty() {
        let mut rng = StdRng::seed_from_u64(73);

        let roll1 = Roll::new(1, DieSize::D20).add_modifier(40);
        let check1 = Check::new(roll1).set_difficulty(30);

        assert!(check1.resolve(&mut rng));

        let roll2 = Roll::new(1, DieSize::D20).add_modifier(-20);
        let check2 = Check::new(roll2).set_difficulty(1);

        assert!(!check2.resolve(&mut rng));
    }
}
