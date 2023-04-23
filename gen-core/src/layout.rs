use arrayvec::ArrayVec;

pub type Key = usize;

pub const ROW_AMOUNT: usize = 3;
pub const COL_AMOUNT: usize = 10;
pub const KEY_AMOUNT: usize = ROW_AMOUNT * COL_AMOUNT;

pub type Matrix = ArrayVec<Key, KEY_AMOUNT>;

pub struct Layout {
    matrix: Matrix,
}

impl Layout {
    pub fn new(keys: [Key; KEY_AMOUNT]) -> Self {
        Self {
            matrix: keys.into(),
        }
    }

    pub fn keys(&self) -> &[Key] {
        &self.matrix
    }
}
