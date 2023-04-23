use crate::{keyboard::*, keyboard_types::*, trigram_types::*, trigrams::*};

pub struct Weights<'a> {
    inner: [f32; 27000], // { KEY_AMOUNT.pow(3) }
    types: TrigramTypes<'a>,
}

// TODO: implement this  as a .toml file
impl<'a> Weights<'a> {
    pub fn unit() -> Self {
        Self {
            inner: [1.0; 27000],
            types: TrigramTypes::default(),
        }
    }

    pub fn new(keyboard: Keyboard) -> Self {
        let user_weights = WeightsConfig::default();

        let types = TrigramTypes::default();

        TrigramClassifications::new(keyboard, types)
            .into_iter()
            .take(100)
            .for_each(|e| println!("{:<9} = {:?}", e.trigram_type, e.fingerings));

        Self::unit()
    }
}

#[derive(Default)]
pub struct TrigramWeight<'a> {
    weight: f64,
    ttype: &'a dyn TrigramType,
}

impl<'a> TrigramWeight<'a> {
    pub const fn new(weight: f64, ttype: &'a dyn TrigramType) -> Self {
        // let weight = f16::from_f64_const(weight);
        Self { weight, ttype }
    }
}

pub struct SplaygramWeight {
    horizontal: f64,
    vertical: f64,
    threshold: f64,
}

pub struct FingerWeight {
    pinky: f64,
    ring: f64,
    middle: f64,
    index: f64,
    weight: f64,
}

pub struct WeightsConfig {
    sfb: f64,
    sfs: f64,
    sft: f64,
    distance_scalar: f64,
    splaygrams: SplaygramWeight,

    inrolls: f64,
    outrolls: f64,
    alternation: f64,
    onehands: f64,
    redirects: f64,
    finger_agility: FingerWeight,
    finger_disalignment: FingerWeight,
    physical_layout: Keyboard,
}

impl const Default for WeightsConfig {
    fn default() -> Self {
        Self {
            sfb: -5.0,
            sfs: -1.5,
            sft: -12.5,
            distance_scalar: 1.0,
            splaygrams: SplaygramWeight {
                horizontal: -2.0,
                vertical: -2.0,
                threshold: 0.0,
            },
            inrolls: 2.0,
            outrolls: 1.8,
            alternation: 0.5,
            onehands: 0.0,
            redirects: -2.0,
            finger_agility: FingerWeight {
                pinky: 8.0,
                ring: 14.0,
                middle: 19.0,
                index: 20.0,
                weight: 1.0,
            },
            finger_disalignment: FingerWeight {
                pinky: -0.4,
                ring: 0.0,
                middle: 0.8,
                index: 0.0,
                weight: 1.0,
            },
            physical_layout: Ansi::keyboard(),
        }
    }
}

impl WeightsConfig {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new<P: AsRef<std::path::Path>>(_: P) -> Self {
        todo!("implement this to actually read from a .toml file")
    }
}
