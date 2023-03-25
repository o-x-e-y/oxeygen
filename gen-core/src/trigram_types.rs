use itertools::Itertools;
use thiserror::*;

use crate::layout_types::*;

#[derive(Debug, Error)]
pub enum TrigramError {
    #[error("Trigram definitions cannot overlap")]
    TrigramOverlapError,
}

#[const_trait]
pub trait TrigramType {
    fn is_type(&self, _: [Finger; 3]) -> bool {
        false
    }
}

#[derive(Default, Debug)]
struct Sfb;

#[derive(Default, Debug)]
struct Sfr;

#[derive(Default, Debug)]
struct Sft;

#[derive(Default, Debug)]
struct Inroll;

#[derive(Default, Debug)]
struct Outroll;

#[derive(Default, Debug)]
struct Alternation;

#[derive(Default, Debug)]
struct Redirect;

#[derive(Default, Debug)]
struct OnehandIn;

#[derive(Default, Debug)]
struct OnehandOut;

#[derive(Default, Debug)]
struct Unspecified;

impl const TrigramType for Sfb {
    fn is_type(&self, [f1, f2, f3]: [Finger; 3]) -> bool {
        let (f1, f2, f3) = (f1 as u8, f2 as u8, f3 as u8);
        (f1 == f2 || f2 == f3) && f1 != f3
    }
}

impl const TrigramType for Sft {
    fn is_type(&self, [f1, f2, f3]: [Finger; 3]) -> bool {
        let (f1, f2, f3) = (f1 as u8, f2 as u8, f3 as u8);
        f1 == f2 && f2 == f3
    }
}

impl const TrigramType for Inroll {
    fn is_type(&self, [f1, f2, f3]: [Finger; 3]) -> bool {
        let (f1, f2, f3) = (f1 as u8, f2 as u8, f3 as u8);
        let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

        (lh1 && lh2 && !lh3) && f1 < f2
            || (!lh1 && lh2 && lh3) && f2 < f3
            || (!lh1 && !lh2 && lh3) && f1 > f2
            || (lh1 && !lh2 && !lh3) && f2 > f3
    }
}

impl const TrigramType for Outroll {
    fn is_type(&self, [f1, f2, f3]: [Finger; 3]) -> bool {
        let (f1, f2, f3) = (f1 as u8, f2 as u8, f3 as u8);
        let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

        (lh1 && lh2 && !lh3) && f1 > f2
            || (!lh1 && lh2 && lh3) && f2 > f3
            || (!lh1 && !lh2 && lh3) && f1 < f2
            || (lh1 && !lh2 && !lh3) && f2 < f3
    }
}

impl const TrigramType for Alternation {
    fn is_type(&self, [f1, f2, f3]: [Finger; 3]) -> bool {
        let (f1, f2, f3) = (f1 as u8, f2 as u8, f3 as u8);
        let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

        lh1 && !lh2 && lh3 || !lh1 && lh2 && !lh3
    }
}

impl const TrigramType for OnehandIn {
    fn is_type(&self, [f1, f2, f3]: [Finger; 3]) -> bool {
        let (f1, f2, f3) = (f1 as u8, f2 as u8, f3 as u8);
        let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

        (lh1 && lh2 && lh3) && (f1 > f2 && f2 > f3) || !(lh1 || lh2 || lh3) && (f1 < f2 && f2 < f3)
    }
}

impl const TrigramType for OnehandOut {
    fn is_type(&self, [f1, f2, f3]: [Finger; 3]) -> bool {
        let (f1, f2, f3) = (f1 as u8, f2 as u8, f3 as u8);
        let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

        (lh1 && lh2 && lh3) && (f1 < f2 && f2 < f3) || !(lh1 || lh2 || lh3) && (f1 > f2 && f2 > f3)
    }
}

impl const TrigramType for Redirect {
    fn is_type(&self, [f1, f2, f3]: [Finger; 3]) -> bool {
        let (f1, f2, f3) = (f1 as u8, f2 as u8, f3 as u8);
        let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

        (lh1 == lh2 && lh2 == lh3) && ((f1 < f2 && f2 > f3) || (f1 > f2 && f2 < f3))
    }
}

impl const TrigramType for Unspecified {}

impl const Default for &dyn TrigramType {
    fn default() -> Self {
        &Unspecified
    }
}

pub struct TrigramTypes<'a> {
    types: Vec<&'a dyn TrigramType>,
    default: &'a dyn TrigramType,
}

impl<'a> Default for TrigramTypes<'a> {
    fn default() -> Self {
        Self {
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
    pub fn match_type(&self, fingers: [Finger; 3]) -> &'a dyn TrigramType {
        *self
            .types
            .iter()
            .find(|e| e.is_type(fingers))
            .unwrap_or(&self.default)
    }

    fn has_overlap(&self) -> bool {
        (LP..=RP)
            .combinations_with_replacement(3)
            .map(|v| [v[0], v[1], v[2]])
            .cartesian_product(&self.types)
            .chunks(self.types.len())
            .into_iter()
            .map(|c| {
                c.into_iter()
                    .filter(|(fingers, t)| t.is_type(*fingers))
                    .count()
            })
            .any(|c| c > 1)
    }

    pub fn new(types: Vec<&'a dyn TrigramType>) -> Result<Self, TrigramError> {
        let new = Self {
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
