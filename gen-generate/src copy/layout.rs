use arrayvec::ArrayVec;
use itertools::Itertools;

pub type Key = usize;

pub const ROW_AMOUNT: usize = 3;
pub const COL_AMOUNT: usize = 10;
pub const KEY_AMOUNT: usize = ROW_AMOUNT * COL_AMOUNT;

pub type Matrix = ArrayVec<Key, KEY_AMOUNT>;

pub struct Layout {
    matrix: Matrix,
}

impl Layout {
    pub fn new(keys: [usize; 30]) -> Self {
        Self {
            matrix: keys.into(),
        }
    }

    pub fn keys(&self) -> &[usize] {
        &self.matrix
    }

    pub fn trigrams(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        self.matrix
            .iter()
            .combinations_with_replacement(3)
            .map(|v| unsafe {
                (
                    **v.get_unchecked(0),
                    **v.get_unchecked(1),
                    **v.get_unchecked(2),
                )
            })
    }
}
