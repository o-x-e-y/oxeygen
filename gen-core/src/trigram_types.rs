use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    iter::once,
};

use itertools::Itertools;
use thiserror::*;

use crate::{
    keyboard::*,
    keyboard_types::{Ansi, KeyboardType}
};

#[derive(Debug, Error)]
pub enum TrigramError {
    #[error("Trigram definitions cannot overlap")]
    TrigramOverlapError,
}

pub trait TrigramType {
    fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool;

    fn display(&self) -> &str;
}

impl Default for &dyn TrigramType {
    fn default() -> Self {
        &Unspecified
    }
}

impl Hash for &dyn TrigramType
where
    dyn TrigramType: TrigramType,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.display().hash(state)
    }
}

impl std::fmt::Display for &dyn TrigramType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

pub mod default {
    use super::*;

    pub struct Sfb;

    pub struct Sfr;

    pub struct Sft;

    pub struct Inroll;

    pub struct Outroll;

    pub struct Alternation;

    pub struct Redirect;

    pub struct OnehandIn;

    pub struct OnehandOut;

    pub struct Unspecified;

    impl TrigramType for Sfb {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingerings(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false
            };
            
            (f1 == f2 || f2 == f3) && f1 != f3
        }

        fn display(&self) -> &str {
            "Sfb"
        }
    }

    impl TrigramType for Sft {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingerings(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false
            };

            f1 == f2 && f2 == f3
        }

        fn display(&self) -> &str {
            "Sft"
        }
    }

    impl TrigramType for Inroll {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingerings(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false
            };

            let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

            (lh1 && lh2 && !lh3) && f1 < f2
                || (!lh1 && lh2 && lh3) && f2 < f3
                || (!lh1 && !lh2 && lh3) && f1 > f2
                || (lh1 && !lh2 && !lh3) && f2 > f3
        }

        fn display(&self) -> &str {
            "Inroll"
        }
    }

    impl TrigramType for Outroll {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingerings(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false
            };

            let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

            (lh1 && lh2 && !lh3) && f1 > f2
                || (!lh1 && lh2 && lh3) && f2 > f3
                || (!lh1 && !lh2 && lh3) && f1 < f2
                || (lh1 && !lh2 && !lh3) && f2 < f3
        }

        fn display(&self) -> &str {
            "Outroll"
        }
    }

    impl TrigramType for Alternation {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingerings(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false
            };

            let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

            lh1 && !lh2 && lh3 || !lh1 && lh2 && !lh3
        }

        fn display(&self) -> &str {
            "Alternation"
        }
    }

    impl TrigramType for OnehandIn {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingerings(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false
            };

            let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

            (lh1 && lh2 && lh3) && (f1 < f2 && f2 < f3)
                || !(lh1 || lh2 || lh3) && (f1 > f2 && f2 > f3)
        }

        fn display(&self) -> &str {
            "Onehand In"
        }
    }

    impl TrigramType for OnehandOut {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingerings(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false
            };

            let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

            (lh1 && lh2 && lh3) && (f1 > f2 && f2 > f3)
                || !(lh1 || lh2 || lh3) && (f1 < f2 && f2 < f3)
        }

        fn display(&self) -> &str {
            "Onehand Out"
        }
    }

    impl TrigramType for Redirect {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingerings(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false
            };

            let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

            (lh1 == lh2 && lh2 == lh3) && ((f1 < f2 && f2 > f3) || (f1 > f2 && f2 < f3))
        }

        fn display(&self) -> &str {
            "Redirect"
        }
    }

    impl TrigramType for Unspecified {
        fn is_type(&self, _: &Keyboard, _: [Pos; 3]) -> bool {
            false
        }

        fn display(&self) -> &str {
            "Unspecified"
        }
    }
}

#[derive(Debug)]
pub struct DynamicType {
    is_type: fn([Finger; 3]) -> bool,
    display: String
}

impl TrigramType for DynamicType {
    fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
        let fingerings = keyboard.get_fingerings(positions);
        let fingers = match fingerings {
            [Some(f1), Some(f2), Some(f3)] => [f1, f2, f3],
            _ => return false
        };

        (self.is_type)(fingers)
    }

    fn display(&self) -> &str {
        self.display.as_str()
    }
}

impl DynamicType {
    pub fn new(is_type: fn([Finger; 3]) -> bool, display: impl Into<String>) -> Self {
        let display = display.into();
        Self { is_type, display }
    }
}

use default::*;

#[derive(Clone)]
pub struct TrigramTypes<'a> {
    keyboard: Keyboard,
    types: Vec<&'a dyn TrigramType>,
    default: &'a dyn TrigramType,
}

impl<'a> Default for TrigramTypes<'a> {
    fn default() -> Self {
        Self {
            keyboard: Ansi::keyboard(),
            types: vec![
                &Sfb,
                &Sft,
                &Inroll,
                &Outroll,
                &Alternation,
                &OnehandIn,
                &OnehandOut,
                &Redirect,
            ],
            default: &Unspecified,
        }
    }
}

impl<'a> TrigramTypes<'a> {
    pub fn unspecified(&self) -> &'a dyn TrigramType {
        self.default
    }

    pub fn match_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> &'a dyn TrigramType {
        *self
            .types
            .iter()
            .find(|e| e.is_type(keyboard, positions))
            .unwrap_or(&self.default)
    }

    fn has_overlap(&self) -> bool {
        let definition_overlap = self.keyboard.positions_iter()
            .cartesian_product(&self.types)
            .chunks(self.types.len())
            .into_iter()
            .map(|c| {
                c.into_iter()
                    .filter(|(positions, t)| t.is_type(&self.keyboard, positions.clone()))
                    .count()
            })
            .any(|c| c > 1);

        let mut hasher = DefaultHasher::new();
        let hash_overlap = self
            .types
            .iter()
            .chain(once(&self.default))
            .map(|t| {
                t.hash(&mut hasher);
                hasher.finish()
            })
            .all_unique();

        definition_overlap && hash_overlap
    }

    pub fn new(types: Vec<&'a dyn TrigramType>, keyboard: Keyboard) -> Result<Self, TrigramError> {
        let new = Self {
            keyboard,
            types,
            default: &Unspecified,
        };

        if !new.has_overlap() {
            Ok(new)
        } else {
            Err(TrigramError::TrigramOverlapError)
        }
    }

    pub fn add_type(&mut self, t: &'a dyn TrigramType) -> Result<(), TrigramError> {
        self.types.push(t);

        if !self.has_overlap() {
            Ok(())
        } else {
            self.types.pop();
            Err(TrigramError::TrigramOverlapError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_no_overlap() {
        let mut default = TrigramTypes::default();

        assert!(!default.has_overlap());

        assert!(default.add_type(&Sfb).is_err());
    }
}
