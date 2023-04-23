#![allow(incomplete_features)]
#![feature(
    const_trait_impl,
    // const_default_impls,
    const_hash,
    iter_array_chunks,
    step_trait,
    // let_chains,
    // slice_index_methods,
    adt_const_params,
    const_mut_refs
)]

pub mod corpus_refiner;
pub mod data;
pub mod keyboard;
pub mod keyboard_types;
pub mod layout;
pub mod trigram_types;
pub mod stats;

pub const REPLACEMENT_CHAR: char = char::REPLACEMENT_CHARACTER;
pub const SHIFT_CHAR: char = '⇑';
pub const REPEAT_KEY: char = '@';

pub mod prelude {
    use super::*;

    pub use corpus_refiner::{CorpusRefiner, RefineCorpus};
    pub use keyboard::Keyboard;
    pub use layout::Layout;
    pub use trigram_types::{default, TrigramType, TrigramTypes};
}

#[cfg(test)]
mod tests {
    // use super::*;
}
