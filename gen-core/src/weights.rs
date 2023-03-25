use std::path::Path;

use arrayvec::ArrayVec;
use half::prelude::*;

use crate::{layout::KEY_AMOUNT, layout_types::*, trigram_types::*};

type WeightsInner = ArrayVec<f16, { KEY_AMOUNT.pow(3) }>;

#[derive(Default)]
pub struct Weights {
    inner: WeightsInner,
}

// TODO: implement this  as a .toml file
impl Weights {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let user_weights = WeightsConfig::default();

        for i1 in 0..30 {
            for i2 in 0..30 {
                for i3 in 0..30 {}
            }
        }
        Self::default()
    }
}

pub struct TrigramClassification<'a> {
    index: usize,
    fingerings: [Finger; 3],
    coordinates: [(f64, f64); 3],
    trigram_type: &'a dyn TrigramType,
}

pub struct TrigramIterator<'a> {
    iteration: usize,
    keyboard: Keyboard,
    types: TrigramTypes<'a>,
}

impl<'a> TrigramIterator<'a> {
    const fn finished(&self) -> bool {
        self.iteration >= 27000
        //matches!(self.iteration, [30, 0, 0])
    }

    fn positions(&self) -> [Pos; 3] {
        let i1 = self.iteration % KEY_AMOUNT;
        let i2 = (self.iteration / KEY_AMOUNT) % KEY_AMOUNT;
        let i3 = (self.iteration / (KEY_AMOUNT * KEY_AMOUNT)) % KEY_AMOUNT;

        unsafe {
            [
                Pos::new_unchecked(i1),
                Pos::new_unchecked(i2),
                Pos::new_unchecked(i3),
            ]
        }
    }
}

pub struct Trigrams<'a> {
    keyboard: Keyboard,
    types: TrigramTypes<'a>,
}

impl<'a> Trigrams<'a> {
    pub fn new(keyboard: Keyboard, types: Vec<&'a dyn TrigramType>) -> Result<Self, TrigramError> {
        let res = Self {
            keyboard,
            types: TrigramTypes::new(types)?,
        };
        Ok(res)
    }

    pub fn default_types(keyboard: Keyboard) -> Self {
        Self {
            keyboard,
            types: TrigramTypes::default(),
        }
    }
}

impl<'a> Iterator for TrigramIterator<'a> {
    type Item = TrigramClassification<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.finished() {
            let positions = <TrigramIterator>::positions(self);
            let fingerings = self.keyboard.get_fingerings(positions);
            let coordinates = self.keyboard.get_coordinates(positions);
            let trigram_type = self.types.match_type(fingerings);

            self.iteration += 1;

            Some(TrigramClassification {
                index: self.iteration,
                fingerings,
                coordinates,
                trigram_type,
            })
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for Trigrams<'a> {
    type Item = TrigramClassification<'a>;

    type IntoIter = TrigramIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TrigramIterator {
            iteration: 0,
            keyboard: self.keyboard,
            types: self.types,
        }
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
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        todo!("implement this to actually read from a .toml file")
    }
}
