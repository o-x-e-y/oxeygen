use std::iter::Step;

use bounded_integer::BoundedUsize;

use crate::layout::KEY_AMOUNT;

pub type Pos = BoundedUsize<0, { KEY_AMOUNT - 1 }>;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Finger {
    LP,
    LR,
    LM,
    LI,
    LT,
    RT,
    RI,
    RM,
    RR,
    RP,
}

impl Finger {
    fn from_int<F: Into<usize>>(int: F) -> Option<Self> {
        match int.into() {
            0 => Some(LP),
            1 => Some(LR),
            2 => Some(LM),
            3 => Some(LI),
            4 => Some(LT),
            5 => Some(RT),
            6 => Some(RI),
            7 => Some(RM),
            8 => Some(RR),
            9 => Some(RP),
            _ => None,
        }
    }
}

impl Step for Finger {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if start > end {
            Some(0)
        } else {
            Some(*end as usize - *start as usize)
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Self::from_int(start as usize + count)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Self::from_int(start as usize - count)
    }
}

pub use Finger::*;

pub struct Fingerings {
    inner: [Finger; KEY_AMOUNT],
}

impl Default for Fingerings {
    #[rustfmt::skip]
    fn default() -> Self {
        Self { inner: [
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
        ]}
    }
}

impl Fingerings {
    pub const fn custom(fingerings: [Finger; KEY_AMOUNT]) -> Self {
        Self { inner: fingerings }
    }

    pub fn get_fingering(&self, pos: Pos) -> Finger {
        self.inner[pos]
    }

    pub fn get_fingerings<const N: usize>(&self, positions: [Pos; N]) -> [Finger; N] {
        let mut res = [Finger::LP; N];

        for i in 0..N {
            let pos = positions[i];
            res[i] = self.get_fingering(pos);
        }
        res
    }

    pub const fn into_inner(&self) -> [Finger; KEY_AMOUNT] {
        self.inner
    }
}

pub struct PhysicalDistances {
    inner: [(f64, f64); KEY_AMOUNT],
}

impl Default for PhysicalDistances {
    #[rustfmt::skip]
    fn default() -> Self {
        Self { inner: [
            (0.0, 0.0),  (1.0, 0.0),  (2.0, 0.0),  (3.0, 0.0),  (4.0, 0.0),  (5.0, 0.0),  (6.0, 0.0),  (7.0, 0.0),  (8.0, 0.0),  (9.0, 0.0),
            (0.25, 1.0), (1.25, 1.0), (2.25, 1.0), (3.25, 1.0), (4.25, 1.0), (5.25, 1.0), (6.25, 1.0), (7.25, 1.0), (8.25, 1.0), (9.25, 1.0),
            (0.5, 2.0),  (1.5, 2.0),  (2.5, 2.0),  (3.5, 2.0),  (4.5, 2.0),  (5.5, 2.0),  (6.5, 2.0),  (7.5, 2.0),  (8.5, 2.0),  (9.0, 0.0),
        ]}
    }
}

impl PhysicalDistances {
    pub const fn custom(distances: [(f64, f64); KEY_AMOUNT]) -> Self {
        Self { inner: distances }
    }

    pub fn get_coordinate(&self, pos: Pos) -> (f64, f64) {
        self.inner[pos]
    }

    pub fn get_coordinates<const N: usize>(&self, positions: [Pos; N]) -> [(f64, f64); N] {
        let mut res = [(0.0, 0.0); N];

        for i in 0..N {
            let pos = positions[i];
            res[i] = self.get_coordinate(pos);
        }
        res
    }

    pub const fn into_inner(&self) -> [(f64, f64); KEY_AMOUNT] {
        self.inner
    }
}

pub const DEFAULT_KEY_SIZE: f64 = 19.05;

pub struct Keyboard {
    distances: PhysicalDistances,
    fingerings: Fingerings,
    key_size: f64,
}

impl Keyboard {
    pub const fn custom(
        distances: PhysicalDistances,
        fingerings: Fingerings,
        key_size: f64,
    ) -> Self {
        Self {
            distances,
            fingerings,
            key_size,
        }
    }

    pub fn get_distance(&self, p1: Pos, p2: Pos) -> f64 {
        let (x1, y1) = self.distances.get_coordinate(p1);
        let (x2, y2) = self.distances.get_coordinate(p2);

        (x1 - x2).hypot(y1 - y2)
    }

    pub fn get_coordinate(&self, pos: Pos) -> (f64, f64) {
        self.distances.get_coordinate(pos)
    }

    pub fn get_coordinates<const N: usize>(&self, positions: [Pos; N]) -> [(f64, f64); N] {
        self.distances.get_coordinates(positions)
    }

    pub fn get_fingering(&self, pos: Pos) -> Finger {
        self.fingerings.get_fingering(pos)
    }

    pub fn get_fingerings<const N: usize>(&self, positions: [Pos; N]) -> [Finger; N] {
        self.fingerings.get_fingerings(positions)
    }
}
