use fxhash::FxHashMap;

use crate::keyboard::Pos;

pub type Matrix = Box<[char]>;

#[derive(Debug, Clone)]
pub struct Layout {
    matrix: Matrix,
    char_to_pos: FxHashMap<char, Pos>,
    score: f32,
}

impl Layout {
    pub fn new<const N: usize>(keys: [char; N]) -> Self {
        let char_to_pos = keys.iter().copied().zip(0..).collect::<FxHashMap<_, _>>();

        Self {
            matrix: Box::new(keys),
            char_to_pos,
            score: 0.0,
        }
    }

    pub fn score(&self) -> f32 {
        self.score
    }

    pub fn set_score(&mut self, score: f32) {
        self.score = score
    }

    pub fn keys(&self) -> &[char] {
        &self.matrix
    }

    pub unsafe fn c(&self, p: Pos) -> char {
        *self.matrix.get_unchecked(p)
    }

    pub unsafe fn trigram(&self, [p1, p2, p3]: [Pos; 3]) -> [char; 3] {
        [self.c(p1), self.c(p2), self.c(p3)]
    }

    pub unsafe fn swap(&mut self, p1: Pos, p2: Pos) {
        let (c1, c2) = (self.c(p1), self.c(p2));

        *self.matrix.get_unchecked_mut(p1) = c2;
        *self.matrix.get_unchecked_mut(p2) = c1;

        self.char_to_pos.entry(c1).and_modify(|e| *e = p2);
        self.char_to_pos.entry(c2).and_modify(|e| *e = p1);
    }
}

impl<const N: usize> From<[char; N]> for Layout {
    fn from(keys: [char; N]) -> Self {
        Self::new(keys)
    }
}
