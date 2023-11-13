pub use libdof::prelude::Finger;
pub use Finger::*;

pub type Pos = usize;

#[derive(Debug, Clone)]
pub struct Fingerings {
    inner: Box<[Finger]>,
}

impl Default for Fingerings {
    #[rustfmt::skip]
    fn default() -> Self {
        Self { inner: Box::new(
            [
                LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
                LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
                LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            ]
        )}
    }
}

impl Fingerings {
    pub fn custom<const N: usize>(fingerings: [Finger; N]) -> Self {
        Self { inner: Box::new(fingerings) }
    }

    pub fn get_fingering(&self, pos: Pos) -> Option<Finger> {
        self.inner.get(pos).copied()
    }

    pub fn get_fingerings<const N: usize>(&self, positions: [Pos; N]) -> [Option<Finger>; N] {
        let mut res = [None; N];

        for i in 0..N {
            let pos = positions[i];
            res[i] = self.get_fingering(pos);
        }
        res
    }

    pub fn inner(&self) -> &[Finger]{
        &self.inner
    }

    pub fn into_inner(self) -> Box<[Finger]> {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalDistances {
    inner: Box<[(f64, f64)]>,
}

impl Default for PhysicalDistances {
    #[rustfmt::skip]
    fn default() -> Self {
        Self { inner: Box::new(
            [
                (0.0, 0.0),  (1.0, 0.0),  (2.0, 0.0),  (3.0, 0.0),  (4.0, 0.0),  (5.0, 0.0),  (6.0, 0.0),  (7.0, 0.0),  (8.0, 0.0),  (9.0, 0.0),
                (0.25, 1.0), (1.25, 1.0), (2.25, 1.0), (3.25, 1.0), (4.25, 1.0), (5.25, 1.0), (6.25, 1.0), (7.25, 1.0), (8.25, 1.0), (9.25, 1.0),
                (0.5, 2.0),  (1.5, 2.0),  (2.5, 2.0),  (3.5, 2.0),  (4.5, 2.0),  (5.5, 2.0),  (6.5, 2.0),  (7.5, 2.0),  (8.5, 2.0),  (9.0, 0.0),
            ]
        )}
    }
}

impl PhysicalDistances {
    pub fn custom<const N: usize>(distances: [(f64, f64); N]) -> Self {
        Self { inner: Box::new(distances) }
    }

    pub fn get_coordinate(&self, pos: Pos) -> Option<&(f64, f64)> {
        self.inner.get(pos)
    }

    pub fn get_coordinates<const N: usize>(&self, positions: [Pos; N]) -> [Option<&(f64, f64)>; N] {
        let mut res = [None; N];

        for i in 0..N {
            let pos = positions[i];
            res[i] = self.get_coordinate(pos);
        }
        res
    }

    pub const fn inner(&self) -> &[(f64, f64)] {
        &self.inner
    }

    pub fn into_inner(self) -> Box<[(f64, f64)]> {
        self.inner
    }
}

pub const DEFAULT_KEY_SIZE: f64 = 19.05;

#[derive(Debug, Clone)]
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

    pub fn get_distance(&self, p1: Pos, p2: Pos) -> Option<f64> {
        let d1 = self.distances.get_coordinate(p1);
        let d2 = self.distances.get_coordinate(p2);

        match (d1, d2) {
            (Some((x1, y1)), Some((x2, y2))) => {
                let dist = (x1 - x2).hypot(y1 - y2);
                Some(dist)
            },
            _ => None
        }
    }

    pub fn get_coordinate(&self, pos: Pos) -> Option<&(f64, f64)> {
        self.distances.get_coordinate(pos)
    }

    pub fn get_coordinates<const N: usize>(&self, positions: [Pos; N]) -> [Option<&(f64, f64)>; N] {
        self.distances.get_coordinates(positions)
    }

    pub fn get_fingering(&self, pos: Pos) -> Option<Finger> {
        self.fingerings.get_fingering(pos)
    }

    pub fn get_fingerings<const N: usize>(&self, positions: [Pos; N]) -> [Option<Finger>; N] {
        self.fingerings.get_fingerings(positions)
    }
}
