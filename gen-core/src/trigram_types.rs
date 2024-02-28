use std::iter::once;

use itertools::Itertools;

use crate::keyboard::*;
use libdof::dofinitions::Finger;

pub trait TrigramType {
    fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool;

    fn display(&self) -> &str;
}

impl Default for &dyn TrigramType {
    fn default() -> Self {
        &Unspecified
    }
}

impl std::fmt::Display for &dyn TrigramType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl std::fmt::Debug for &dyn TrigramType {
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
            let fingerings = keyboard.get_fingers(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false,
            };

            !Sfr.is_type(keyboard, positions) && ((f1 == f2 || f2 == f3) && f1 != f3)
        }

        fn display(&self) -> &str {
            "Sfb"
        }
    }

    impl TrigramType for Sfr {
        fn is_type(&self, _: &Keyboard, [p1, p2, p3]: [Pos; 3]) -> bool {
            p1 == p2 || p2 == p3
        }

        fn display(&self) -> &str {
            "Sfr"
        }
    }

    impl TrigramType for Sft {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingers(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false,
            };

            !Sfr.is_type(keyboard, positions) && (f1 == f2 && f2 == f3)
        }

        fn display(&self) -> &str {
            "Sft"
        }
    }

    impl TrigramType for Inroll {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingers(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false,
            };

            let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

            // (lh1 && lh2 && !lh3) && f1 < f2
            //     || (!lh1 && lh2 && lh3) && f2 < f3
            //     || (!lh1 && !lh2 && lh3) && f1 > f2
            //     || (lh1 && !lh2 && !lh3) && f2 > f3

            (f2 > f3 || lh3 || lh2)
                && (f1 > f2 || lh2 || lh1)
                && (f2 < f3 || !lh2 || lh1)
                && (f1 < f2 || lh3 || !lh2)
                && (lh3 || lh1)
                && (!lh3 || !lh1)
        }

        fn display(&self) -> &str {
            "Inroll"
        }
    }

    impl TrigramType for Outroll {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingers(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false,
            };

            let [lh1, lh2, lh3] = [f1 < 5, f2 < 5, f3 < 5];

            // (lh1 && lh2 && !lh3) && f1 > f2
            //     || (!lh1 && lh2 && lh3) && f2 > f3
            //     || (!lh1 && !lh2 && lh3) && f1 < f2
            //     || (lh1 && !lh2 && !lh3) && f2 < f3

            (f2 < f3 || lh3 || lh2)
                && (f1 < f2 || lh2 || lh1)
                && (f2 > f3 || !lh2 || lh1)
                && (f1 > f2 || lh3 || !lh2)
                && (lh3 || lh1)
                && (!lh3 || !lh1)
        }

        fn display(&self) -> &str {
            "Outroll"
        }
    }

    impl TrigramType for Alternation {
        fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
            let fingerings = keyboard.get_fingers(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false,
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
            let fingerings = keyboard.get_fingers(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false,
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
            let fingerings = keyboard.get_fingers(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false,
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
            let fingerings = keyboard.get_fingers(positions);
            let (f1, f2, f3) = match fingerings {
                [Some(f1), Some(f2), Some(f3)] => (f1 as u8, f2 as u8, f3 as u8),
                _ => return false,
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
    display: String,
}

impl TrigramType for DynamicType {
    fn is_type(&self, keyboard: &Keyboard, positions: [Pos; 3]) -> bool {
        let fingerings = keyboard.get_fingers(positions);
        let fingers = match fingerings {
            [Some(f1), Some(f2), Some(f3)] => [f1, f2, f3],
            _ => return false,
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

impl<'a> TrigramTypes<'a> {
    pub fn default(&self) -> &dyn TrigramType {
        self.default
    }

    pub fn types(&self) -> &Vec<&dyn TrigramType> {
        &self.types
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn get_type(&'a self, positions: [Pos; 3]) -> &'a dyn TrigramType {
        *self
            .types()
            .iter()
            .find(|e| e.is_type(&self.keyboard, positions))
            .unwrap_or(&self.default())
    }

    fn has_overlap(&self) -> bool {
        for i in 0..self.keyboard.len() {
            for j in 0..self.keyboard.len() {
                for k in 0..self.keyboard.len() {
                    let c = self
                        .types()
                        .iter()
                        .filter(|t| t.is_type(&self.keyboard, [i, j, k]))
                        .count();
                    if c > 1 {
                        let types = self
                            .types()
                            .iter()
                            .filter(|t| t.is_type(&self.keyboard, [i, j, k]))
                            .map(|t| t.display())
                            .collect::<Vec<_>>();
                        eprintln!("{types:?} overlap at [{i}, {j}, {k}]");
                        return true;
                    }
                }
            }
        }

        !self
            .types()
            .iter()
            .chain(once(&self.default()))
            .map(|t| t.display())
            .all_unique()
    }

    pub fn new(types: Vec<&'a dyn TrigramType>, keyboard: Keyboard) -> Option<Self> {
        let new = Self {
            keyboard,
            types,
            default: &Unspecified,
        };

        if !new.has_overlap() {
            Some(new)
        } else {
            None
        }
    }

    pub fn with_defaults(keyboard: Keyboard) -> Self {
        let types: Vec<&dyn TrigramType> = vec![
            &Sfb,
            &Sfr,
            &Sft,
            &Inroll,
            &Outroll,
            &Alternation,
            &OnehandIn,
            &OnehandOut,
            &Redirect,
        ];
        let default = &Unspecified;

        Self {
            keyboard,
            types,
            default,
        }
    }

    pub fn add_type(&'a mut self, t: &'a dyn TrigramType) -> Option<()> {
        self.types.push(t);

        if !self.has_overlap() {
            Some(())
        } else {
            self.types.pop();
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libdof::dofinitions::Finger::*;

    #[test]
    fn default_no_overlap() {
        #[rustfmt::skip]
        let fingering = [
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
        ];

        let keyboard = Keyboard::new(&fingering);

        let mut default = TrigramTypes::with_defaults(keyboard);

        assert!(!default.has_overlap());

        assert!(default.add_type(&Sfb).is_none());
    }
}
