#![allow(incomplete_features)]
#![feature(
    const_trait_impl,
    const_default_impls,
    step_trait,
    // let_chains,
    // slice_index_methods,
    adt_const_params,
    const_mut_refs
)]

pub mod corpus;
pub mod corpus_refiner;
pub mod data;
pub mod layout;
pub mod layout_types;
pub mod trigram_types;
pub mod weights;

pub const REPLACEMENT_CHAR: char = char::REPLACEMENT_CHARACTER;
pub const SHIFT_CHAR: char = '⇑';
pub const REPEAT_KEY: char = '@';

pub mod prelude {
    use super::*;

    pub use corpus::Corpus;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thing() {
        println!("{}", 3usize.next_power_of_two());
    }
}
