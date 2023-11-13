use fxhash::FxHashMap as HashMap;

pub struct KeyboardPos {}

pub struct Layout<const L: usize> {
    layers: [Vec<char>; L],
    char_to_pos: HashMap<char, (usize, usize)>,
}
