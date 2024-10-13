//! This module provides functionality for generating random strings based on specified criteria.
//!
//! The [`StringDef`] struct defines the configuration for string generation, including options
//! for length, character types, and more. The [`StringDefBuilder`] allows for a convenient way
//! to build and customize a [`StringDef`] instance. The module also includes various utility
//! functions to check for specific character types in a string.

use std::cell::RefCell;

use rand::prelude::IteratorRandom;
use rand::{Rng, RngCore};

use crate::Randomizer;

const SYMBOLS: &str = r##"!\"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"##;

/// Defines the criteria for generating random strings.
#[derive(Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct StringDef {
    /// The desired length of the generated string.
    pub length: u32,
    /// Whether to include Unicode characters in the generated string.
    pub include_unicode: bool,
    /// Whether to include symbols in the generated string.
    pub include_symbol: bool,
    /// Whether to include capital letters in the generated string.
    pub include_capital_letters: bool,
    /// Whether to include numeric characters in the generated string.
    pub include_numbers: bool,
}

/// Provides a builder for constructing a [`StringDef`] instance.
impl Default for StringDef {
    fn default() -> Self {
        Self {
            length: 6,
            include_unicode: false,
            include_symbol: false,
            include_capital_letters: false,
            include_numbers: false,
        }
    }
}

/// A builder for configuring a [`StringDef`].
pub struct StringDefBuilder<'a> {
    pub string_def: StringDef,
    pub rng: &'a RefCell<dyn RngCore + Send>,
}

impl StringDefBuilder<'_> {
    /// Sets the length of the generated string.
    #[must_use]
    pub const fn length(mut self, length: u32) -> Self {
        self.string_def.length = length;
        self
    }

    /// Specifies whether to include Unicode characters.
    #[must_use]
    pub const fn include_unicode(mut self, yes: bool) -> Self {
        self.string_def.include_unicode = yes;
        self
    }

    /// Specifies whether to include symbols.
    #[must_use]
    pub const fn include_symbol(mut self, yes: bool) -> Self {
        self.string_def.include_symbol = yes;
        self
    }

    /// Specifies whether to include capital letters.
    #[must_use]
    pub const fn include_capital_letters(mut self, yes: bool) -> Self {
        self.string_def.include_capital_letters = yes;
        self
    }

    /// Specifies whether to include numbers.
    #[must_use]
    pub const fn include_numbers(mut self, yes: bool) -> Self {
        self.string_def.include_numbers = yes;
        self
    }
}

impl std::fmt::Display for StringDefBuilder<'_> {
    /// Displays the generated string based on the current configuration of the builder.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut rng = self.rng.borrow_mut();
        let result = self.string_def.generate(&mut *rng);
        write!(f, "{result}")
    }
}

impl StringDef {
    /// Creates a [`StringDef`] from a given [`Randomizer`].
    pub fn from_randomizer(randomizer: &Randomizer) -> Self {
        Self {
            length: randomizer.number_between(1, 50),
            include_unicode: randomizer.bool(),
            include_symbol: randomizer.bool(),
            include_capital_letters: randomizer.bool(),
            include_numbers: randomizer.bool(),
        }
    }

    /// Generates a random string based on the current configuration.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crazy_train::{Randomizer, StringDef};
    /// let string_def = StringDef::default();
    /// let randomizer = Randomizer::with_seed(42);
    /// let mut rng = randomizer.rng.borrow_mut();
    /// assert_eq!(string_def.generate(&mut *rng), "noqkak");
    /// assert_eq!(string_def.generate(&mut *rng), "twdayn");
    /// assert_eq!(string_def.generate(&mut *rng), "kdnfan");
    /// ```
    pub fn generate(&self, rng: &mut dyn RngCore) -> String {
        let mut result = String::new();
        let length: usize = self.length as usize;

        while result.len() < length {
            let choice: u8 = rng.gen_range(0..100);

            if self.include_unicode && choice < 20 {
                if let Some(unicode_char) = std::char::from_u32(rng.gen_range(0x1F600..0x1F64F)) {
                    result.push(unicode_char);
                } else {
                    result.push('?');
                }
            } else if self.include_symbol && choice < 40 {
                if let Some(symbol) = SYMBOLS.chars().choose(rng) {
                    result.push(symbol);
                } else {
                    result.push('#');
                }
            } else if self.include_capital_letters && choice < 60 {
                let capital_letter = rng.gen_range(b'A'..=b'Z') as char;
                result.push(capital_letter);
            } else if self.include_numbers && choice < 80 {
                let number = rng.gen_range(b'0'..=b'9') as char;
                result.push(number);
            } else {
                let lowercase_letter = rng.gen_range(b'a'..=b'z') as char;
                result.push(lowercase_letter);
            }
        }

        result
    }

    /// Checks if a given string contains only lowercase letters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crazy_train::StringDef;
    /// assert!(StringDef::contains_only_lowercase("test"));
    /// assert!(!StringDef::contains_only_lowercase("1test"));
    /// assert!(!StringDef::contains_only_lowercase("🙆test"));
    /// assert!(!StringDef::contains_only_lowercase("#test"));
    /// assert!(!StringDef::contains_only_lowercase("Test"));
    /// ```
    #[must_use]
    pub fn contains_only_lowercase(s: &str) -> bool {
        !Self::contains_capital_letters(s)
            && !Self::contains_numbers(s)
            && !Self::contains_symbols(s)
            && !Self::contains_unicode(s)
    }

    /// Checks if a given string contains any Unicode characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crazy_train::StringDef;
    /// assert!(!StringDef::contains_unicode("test"));
    /// assert!(StringDef::contains_unicode("🙆Test"));
    /// ```
    #[must_use]
    pub fn contains_unicode(s: &str) -> bool {
        s.chars().any(|ch| !ch.is_ascii())
    }

    /// Checks if a given string contains any symbols.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crazy_train::StringDef;
    /// assert!(!StringDef::contains_symbols("test"));
    /// assert!(StringDef::contains_symbols("#Test"));
    /// ```
    #[must_use]
    pub fn contains_symbols(s: &str) -> bool {
        s.chars().any(|ch| SYMBOLS.contains(ch))
    }

    /// Checks if a given string contains any numeric characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crazy_train::StringDef;
    /// assert!(!StringDef::contains_numbers("test"));
    /// assert!(StringDef::contains_numbers("1Test"));
    /// ```
    #[must_use]
    pub fn contains_numbers(s: &str) -> bool {
        s.chars().any(char::is_numeric)
    }

    /// Checks if a given string contains any capital letters.
    /// # Example
    ///
    /// ```rust
    /// use crazy_train::StringDef;
    /// assert!(!StringDef::contains_capital_letters("test"));
    /// assert!(StringDef::contains_capital_letters("Test"));
    /// ```
    #[must_use]
    pub fn contains_capital_letters(s: &str) -> bool {
        s.chars().any(char::is_uppercase)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn has_unicode() {
        assert!(!StringDef::contains_unicode("test"));
        assert!(StringDef::contains_unicode("🙆test"));
    }

    #[test]
    fn has_symbols() {
        assert!(!StringDef::contains_symbols("test"));
        assert!(StringDef::contains_symbols("test#"));
    }

    #[test]
    fn has_numbers() {
        assert!(!StringDef::contains_numbers("test"));
        assert!(StringDef::contains_numbers("test1"));
    }

    #[test]
    fn has_capital_letters() {
        assert!(!StringDef::contains_capital_letters("test"));
        assert!(StringDef::contains_capital_letters("Test1"));
    }

    #[test]
    fn string_def_default() {
        let string_def = StringDef::default();
        let randomizer = Randomizer::with_seed(42);
        let mut rng = randomizer.rng.borrow_mut();
        assert_eq!(string_def.generate(&mut *rng), "noqkak");
        assert_eq!(string_def.generate(&mut *rng), "twdayn");
        assert_eq!(string_def.generate(&mut *rng), "kdnfan");
    }

    #[test]
    fn string_def_with_length() {
        let string_def = StringDef {
            length: 10,
            include_unicode: false,
            include_symbol: false,
            include_capital_letters: false,
            include_numbers: false,
        };
        let mut rand = Box::new(StdRng::seed_from_u64(42));
        assert_eq!(string_def.generate(&mut rand), "noqkaktwda");
        assert_eq!(string_def.generate(&mut rand), "ynkdnfanbq");
        assert_eq!(string_def.generate(&mut rand), "vmnbjlufkr");
    }

    #[test]
    fn string_def_include_unicode() {
        let string_def = StringDef {
            length: 6,
            include_unicode: true,
            include_symbol: false,
            include_capital_letters: false,
            include_numbers: false,
        };
        let mut rand = Box::new(StdRng::seed_from_u64(42));
        assert_eq!(string_def.generate(&mut rand), "😩oq");
        assert_eq!(string_def.generate(&mut rand), "kakt🙃");
        assert_eq!(string_def.generate(&mut rand), "daynkd");
    }

    #[test]
    fn string_def_include_symbol() {
        let string_def = StringDef {
            length: 6,
            include_unicode: false,
            include_symbol: true,
            include_capital_letters: false,
            include_numbers: false,
        };
        let mut rand = Box::new(StdRng::seed_from_u64(42));
        assert_eq!(string_def.generate(&mut rand), "\"eq)a)");
        assert_eq!(string_def.generate(&mut rand), "=wqf`g");
        assert_eq!(string_def.generate(&mut rand), "/uzw=d");
    }

    #[test]
    fn string_def_include_capital_letters() {
        let string_def = StringDef {
            length: 6,
            include_unicode: false,
            include_symbol: false,
            include_capital_letters: true,
            include_numbers: false,
        };
        let mut rand = Box::new(StdRng::seed_from_u64(42));
        assert_eq!(string_def.generate(&mut rand), "NOqkak");
        assert_eq!(string_def.generate(&mut rand), "TWdAyN");
        assert_eq!(string_def.generate(&mut rand), "kdnfaN");
    }

    #[test]
    fn string_def_include_numbers() {
        let string_def = StringDef {
            length: 6,
            include_unicode: false,
            include_symbol: false,
            include_capital_letters: false,
            include_numbers: true,
        };
        let mut rand = Box::new(StdRng::seed_from_u64(42));
        assert_eq!(string_def.generate(&mut rand), "55qka4");
        assert_eq!(string_def.generate(&mut rand), "7810y5");
        assert_eq!(string_def.generate(&mut rand), "k1nf05");
    }
}
