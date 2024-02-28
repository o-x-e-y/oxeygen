pub mod corpus_refiner;
pub mod data;
pub mod keyboard;
pub mod layout;
pub mod mapping;
pub mod optimizer;
pub mod prelude;
pub mod trigram_types;
pub mod weights;

pub use libdof;

pub const REPLACEMENT_CHAR: char = char::REPLACEMENT_CHARACTER;
pub const SHIFT_CHAR: char = '⇑';
pub const REPEAT_KEY: char = '@';
