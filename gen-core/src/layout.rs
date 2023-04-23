pub const ROW_AMOUNT: usize = 3;
pub const COL_AMOUNT: usize = 10;
pub const KEY_AMOUNT: usize = ROW_AMOUNT * COL_AMOUNT;

pub type Matrix = [char; KEY_AMOUNT];

pub struct Layout {
    matrix: Matrix,
}

impl Layout {
    pub fn new(keys: [char; KEY_AMOUNT]) -> Self {
        Self {
            matrix: keys,
        }
    }

    pub fn keys(&self) -> &[char] {
        &self.matrix
    }
}
