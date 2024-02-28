use libdof::dofinitions::Finger;
use nanorand::{Rng, WyRand};

use crate::keyboard::Pos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    keys: Box<[usize]>,
    fingers: Box<[Finger]>,
}

impl Layout {
    pub fn new(keys: &[usize], fingering: &[Finger]) -> Option<Self> {
        if keys.len() == fingering.len() {
            let keys = keys.to_vec().into();
            let fingers = fingering.to_vec().into();

            Some(Self { keys, fingers })
        } else {
            None
        }
    }

    pub fn from_vecs(keys: Vec<usize>, fingering: Vec<Finger>) -> Option<Self> {
        if keys.len() == fingering.len() {
            let keys = keys.into();
            let fingers = fingering.into();

            Some(Self { keys, fingers })
        } else {
            None
        }
    }

    pub fn random(mut keys: Vec<usize>, fingering: Vec<Finger>) -> Option<Self> {
        let mut rng = WyRand::new();
        rng.shuffle(&mut keys);

        Self::from_vecs(keys, fingering)
    }

    pub fn fingers(&self) -> &[Finger] {
        &self.fingers
    }

    pub fn keys(&self) -> &[usize] {
        &self.keys
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn finger(&self, i: Pos) -> Option<Finger> {
        self.fingers.get(i).copied()
    }

    pub fn key(&self, i: Pos) -> Option<usize> {
        self.keys.get(i).copied()
    }

    pub fn finger_trigram(&self, [t1, t2, t3]: [Pos; 3]) -> Option<[Finger; 3]> {
        let fingers = match (self.finger(t1), self.finger(t2), self.finger(t3)) {
            (Some(f1), Some(f2), Some(f3)) => [f1, f2, f3],
            _ => return None,
        };

        Some(fingers)
    }

    pub fn key_trigram(&self, [t1, t2, t3]: [Pos; 3]) -> Option<[usize; 3]> {
        let keys = match (self.key(t1), self.key(t2), self.key(t3)) {
            (Some(f1), Some(f2), Some(f3)) => [f1, f2, f3],
            _ => return None,
        };

        Some(keys)
    }

    pub(crate) unsafe fn k(&self, i: Pos) -> usize {
        *self.keys.get_unchecked(i)
    }

    pub(crate) unsafe fn _f(&self, i: Pos) -> Finger {
        *self.fingers.get_unchecked(i)
    }

    pub(crate) unsafe fn kt(&self, [t1, t2, t3]: [Pos; 3]) -> [usize; 3] {
        [self.k(t1), self.k(t2), self.k(t3)]
    }

    pub(crate) unsafe fn _ft(&self, [t1, t2, t3]: [Pos; 3]) -> [Finger; 3] {
        [self._f(t1), self._f(t2), self._f(t3)]
    }

    pub(crate) unsafe fn swap(&mut self, p1: Pos, p2: Pos) {
        let help = *self.keys.get_unchecked(p1);

        *self.keys.get_unchecked_mut(p1) = *self.keys.get_unchecked(p2);
        *self.keys.get_unchecked_mut(p2) = help;
    }
}
