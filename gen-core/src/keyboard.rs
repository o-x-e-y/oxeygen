use libdof::dofinitions::Finger;

pub type Pos = usize;

#[derive(Debug, Clone)]
pub struct Keyboard {
    fingers: Box<[Finger]>,
}

impl Keyboard {
    pub fn new(fingers: &[Finger]) -> Self {
        let fingers = fingers.into();

        Self { fingers }
    }

    pub fn fingering(&self) -> &[Finger] {
        &self.fingers
    }

    pub fn get_finger(&self, pos: Pos) -> Option<Finger> {
        self.fingers.get(pos).copied()
    }

    pub fn get_fingers<const N: usize>(&self, positions: [Pos; N]) -> [Option<Finger>; N] {
        let mut res = [None; N];

        for i in 0..N {
            let pos = positions[i];
            res[i] = self.get_finger(pos);
        }
        res
    }

    pub(crate) unsafe fn get_fs<const N: usize>(&self, positions: [Pos; N]) -> [Finger; N] {
        let mut res = [Finger::LP; N];

        for (pos, finger) in positions.into_iter().zip(res.iter_mut()) {
            *finger = *self.fingers.get_unchecked(pos);
        }
        res
    }

    pub fn len(&self) -> usize {
        self.fingers.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
