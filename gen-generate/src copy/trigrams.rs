use std::collections::HashMap;

use fxhash::FxHashMap;
use itertools::Itertools;

use crate::{
    corpus::Corpus,
    keyboard::*,
    layout::{Layout, KEY_AMOUNT},
    trigram_types::*,
};

pub struct TrigramClassification<'a> {
    pub(crate) index: usize,
    pub(crate) fingerings: [Finger; 3],
    pub(crate) coordinates: [(f64, f64); 3],
    pub(crate) trigram_type: &'a dyn TrigramType,
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
                trigram_type: trigram_type,
            })
        } else {
            None
        }
    }
}

pub struct TrigramClassifications<'a> {
    keyboard: Keyboard,
    types: TrigramTypes<'a>,
}

impl<'a> TrigramClassifications<'a> {
    pub fn default_types(keyboard: Keyboard) -> Self {
        Self {
            keyboard,
            types: TrigramTypes::default(),
        }
    }

    pub fn new(keyboard: Keyboard, types: TrigramTypes<'a>) -> Self {
        Self { keyboard, types }
    }
}

impl<'a> IntoIterator for TrigramClassifications<'a> {
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

pub struct KeyboardTrigrams {
    pub(crate) which: [u8; 27000],
    from: HashMap<&'static str, u8>,
    to: HashMap<u8, &'static str>,
}

impl core::ops::Index<(usize, usize, usize)> for KeyboardTrigrams {
    type Output = str;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        assert!(index.0 < 30 && index.1 < 30 && index.2 < 30);

        let i1 = index.0 * 30usize.pow(2);
        let i2 = index.1 * 30;
        let i3 = index.2;

        let index = i1 + i2 + i3;

        self.to(self.which[index]).unwrap()
    }
}

impl<'a> FromIterator<TrigramClassification<'a>> for KeyboardTrigrams {
    fn from_iter<T: IntoIterator<Item = TrigramClassification<'a>>>(iter: T) -> Self {
        let mut which = [0; 27000];
        let mut from = HashMap::new();
        let mut to = HashMap::new();

        for (i, t) in iter.into_iter().enumerate() {
            if !from.contains_key(t.trigram_type.display()) {
                from.insert(t.trigram_type.display(), from.len() as u8);
                to.insert(to.len() as u8, t.trigram_type.display());
            }

            which[i] = *from.get(t.trigram_type.display()).unwrap();
        }

        Self { which, from, to }
    }
}

impl KeyboardTrigrams {
    pub fn new(keyboard: Keyboard, trigram_types: TrigramTypes) -> Self {
        TrigramClassifications::new(keyboard, trigram_types)
            .into_iter()
            .collect()
    }

    pub fn to(&self, encoded: u8) -> Option<&'static str> {
        self.to.get(&encoded).copied()
    }

    pub fn from(&self, decoded: &dyn TrigramType) -> Option<u8> {
        self.from.get(decoded.display()).copied()
    }

    pub fn stat_map(&self, corpus: &Corpus, layout: &Layout) -> HashMap<u8, f32> {
        let mut stat_map = self.to.keys().map(|k| (*k, 0.0)).collect::<HashMap<_, _>>();

        corpus
            .layout_trigrams(layout)
            .zip(self.which)
            .for_each(|(f, t)| {
                stat_map.entry(t).and_modify(|e| *e += f).or_insert(f);
            });

        stat_map
    }
}
