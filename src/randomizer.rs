//! This module provides a Randomizer struct that manages random number generation
//! with support for seeded and non-seeded generation. It allows for generating
//! random numbers, booleans, strings, paths, and shuffling items.
//!

use crate::generator::{StringDef, StringDefBuilder};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, RngCore, SeedableRng};
use std::{cell::RefCell, path::PathBuf};

/// Struct for managing random number generation, allowing seed control for reproducibility.
pub struct Randomizer {
    pub rng: RefCell<Box<dyn RngCore + Send>>,
    pub seed: u64,
}

/// Default implementation for [`Randomizer`], initializing RNG with a random seed.
impl Default for Randomizer {
    fn default() -> Self {
        let mut seed_rng = StdRng::from_entropy();
        let seed = seed_rng.next_u64();

        let rng = RefCell::new(Box::new(StdRng::seed_from_u64(seed)));

        Self { rng, seed }
    }
}

impl Randomizer {
    /// Create a new [`Randomizer`] with a specified seed.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use crazy_train::Randomizer;
    /// let randomizer = Randomizer::with_seed(42);
    /// ```
    #[must_use]
    pub fn with_seed(seed: u64) -> Self {
        let rng = RefCell::new(Box::new(StdRng::seed_from_u64(seed)));
        Self { rng, seed }
    }

    /// Generate a random number between the specified minimum and maximum values (inclusive).
    ///
    /// # Example:
    ///
    /// ```rust
    /// use crazy_train::Randomizer;
    /// let randomizer = Randomizer::with_seed(42);
    /// assert_eq!(randomizer.number_between(1,10), 7);
    //  assert_eq!(!randomizer.number_between(1,10), 2);
    /// ```
    pub fn number_between(&self, min: u32, max: u32) -> u32 {
        let mut rng = self.rng.borrow_mut();
        let random_number = rng.next_u32();
        min + (random_number % (max - min + 1))
    }

    /// Generate a random boolean value (true or false).
    ///
    /// # Example:
    ///
    /// ```rust
    /// use crazy_train::Randomizer;
    /// let randomizer = Randomizer::with_seed(42);
    /// assert!(randomizer.bool());
    //  assert!(!randomizer.bool());
    /// ```
    pub fn bool(&self) -> bool {
        let mut rng = self.rng.borrow_mut();
        let random_number = rng.next_u32();
        random_number % 2 == 0
    }

    /// Create a [`StringDefBuilder`] based on a given [`StringDef`].
    ///
    /// # Example:
    ///
    /// ```rust
    /// use crazy_train::{Randomizer, StringDef};
    /// let string_def = StringDef::default();
    /// let randomizer = Randomizer::with_seed(42);
    /// assert_eq!(randomizer.string(string_def.clone()).to_string(), "noqkak");
    /// assert_eq!(randomizer.string(string_def.clone()).include_capital_letters(true).to_string(), "TWdAyN");
    /// assert_eq!(randomizer.string(string_def.clone()).include_unicode(true).to_string(), "kdnfa😩");
    /// assert_eq!(randomizer.string(string_def.clone()).include_numbers(true).to_string(), "0684n0");
    /// assert_eq!(randomizer.string(string_def.clone()).include_symbol(true).to_string(), "=wqf`g");
    /// assert_eq!(randomizer.string(string_def.clone()).length(10).to_string(), "wgavmyyuzw");
    /// ```
    pub fn string(&self, def: StringDef) -> StringDefBuilder {
        StringDefBuilder {
            string_def: def,
            rng: &self.rng,
        }
    }

    /// Generate a random path of a specified length.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use crazy_train::Randomizer;
    /// use std::path::PathBuf;
    /// let randomizer = Randomizer::with_seed(42);
    /// assert_eq!(randomizer.path(), PathBuf::from("gowqzkza"));
    /// ```
    pub fn path(&self) -> PathBuf {
        let mut rng = self.rng.borrow_mut();

        let path_length = rng.gen_range(5..=10);
        let path_name: String = (0..path_length)
            .map(|_| char::from(rng.gen_range(b'a'..=b'z')))
            .collect();

        PathBuf::from(path_name)
    }

    /// Shuffle a slice of items and return a new vector with the shuffled items.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use crazy_train::Randomizer;
    /// let randomizer = Randomizer::with_seed(42);
    /// let list = vec![1, 2, 3, 4, 5, 6];
    /// assert_eq!(randomizer.shuffle(&list), vec![1, 5, 6, 3, 2, 4]);
    /// ```
    pub fn shuffle<T>(&self, items: &[T]) -> Vec<T>
    where
        T: Clone,
    {
        let mut rng = self.rng.borrow_mut();
        let mut shuffled_items = items.to_vec();
        shuffled_items.shuffle(&mut *rng);
        shuffled_items
    }

    /// Pick a random selection of items from a given slice.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use crazy_train::Randomizer;
    /// let randomizer = Randomizer::with_seed(42);
    /// let list = vec![1, 2, 3, 4, 5, 6];
    /// assert_eq!(randomizer.pick_random(&list), vec![2, 6]);
    /// ```
    pub fn pick_random<T>(&self, items: &[T]) -> Vec<T>
    where
        T: Clone,
    {
        let mut rng = self.rng.borrow_mut();

        let count = rng.gen_range(1..=10);

        (0..count)
            .map(|_| {
                let index = rng.gen_range(0..items.len());
                items[index].clone()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rand_number() {
        let randomizer = Randomizer::with_seed(42);
        assert_eq!(randomizer.number_between(1, 100), 27);
        assert_eq!(randomizer.number_between(1, 100), 52);
        assert_eq!(randomizer.number_between(1, 100), 98);
    }

    #[test]
    fn rand_bool() {
        let randomizer = Randomizer::with_seed(42);
        assert!(randomizer.bool());
        assert!(!randomizer.bool());
        assert!(!randomizer.bool());
        assert!(!randomizer.bool());
        assert!(!randomizer.bool());
        assert!(!randomizer.bool());
        assert!(randomizer.bool());
    }

    #[test]
    fn rand_string() {
        let randomizer = Randomizer::with_seed(42);
        assert_eq!(
            randomizer.string(StringDef::default()).to_string(),
            "noqkak".to_string()
        );
        assert_eq!(
            randomizer.string(StringDef::default()).to_string(),
            "twdayn".to_string()
        );
        assert_eq!(
            randomizer
                .string(StringDef {
                    include_symbol: true,
                    ..Default::default()
                })
                .to_string(),
            "kdnfa)".to_string()
        );

        assert_eq!(
            randomizer
                .string(StringDef {
                    length: 10,
                    ..Default::default()
                })
                .to_string(),
            "hdpnewfykq".to_string()
        );
        assert_eq!(
            randomizer
                .string(StringDef {
                    include_unicode: true,
                    ..Default::default()
                })
                .to_string(),
            "vjjp😓".to_string()
        );
    }

    #[test]
    fn rand_path() {
        let randomizer = Randomizer::with_seed(42);
        assert_eq!(randomizer.path(), PathBuf::from("gowqzkza"));
    }

    #[test]
    fn shuffle() {
        let randomizer = Randomizer::with_seed(42);
        let list = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(randomizer.shuffle(&list), vec![1, 5, 6, 3, 2, 4]);
    }

    #[test]
    fn pick_random() {
        let randomizer = Randomizer::with_seed(42);
        let list = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(randomizer.pick_random(&list), vec![2, 6]);
        assert_eq!(randomizer.pick_random(&list), vec![3, 1, 3, 5, 6, 1, 6]);
    }
}
