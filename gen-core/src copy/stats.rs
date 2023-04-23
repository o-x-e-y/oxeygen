use itertools::Itertools;

use crate::{
    corpus::Corpus, keyboard::Keyboard, layout::Layout, trigram_types::TrigramTypes,
    trigrams::KeyboardTrigrams,
};

pub struct KeyboardStats {
    keyboard_trigrams: KeyboardTrigrams,
    corpus: Corpus,
}

impl KeyboardStats {
    pub fn new(keyboard: Keyboard, trigram_types: TrigramTypes, corpus: Corpus) -> Self {
        Self {
            keyboard_trigrams: KeyboardTrigrams::new(keyboard, trigram_types),
            corpus,
        }
    }

    pub fn default_types(keyboard: Keyboard, corpus: Corpus) -> Self {
        Self::new(keyboard, TrigramTypes::default(), corpus)
    }

    pub fn layout_with(&self, keys: [char; 30]) -> Layout {
        self.corpus.layout_with(keys)
    }

    pub fn print_stats(&self, layout: &Layout) {
        let stat_map = self.keyboard_trigrams.stat_map(&self.corpus, layout);

        let layout = self
            .corpus
            .decode_slice(layout.keys())
            .array_chunks::<10>()
            .map(|r| r.into_iter().intersperse(' ').collect::<String>())
            .intersperse("\n".into())
            .collect::<String>();

        println!("{}", layout);

        for (stat, freq) in stat_map.into_iter() {
            let stat = self.keyboard_trigrams.to(stat).unwrap();
            println!("{:<12} {}", format!("{}:", stat), freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::keyboard_types::{IsoAngle, KeyboardType};

    use super::*;

    #[test]
    fn trigrams() {
        let keyboard = IsoAngle::keyboard();
        let corpus = Corpus::load("../data/akl.json").expect("this should always exist");
        let stats = KeyboardStats::default_types(keyboard, corpus);

        let layout_keys: [char; 30] = "bgdlzjfou,nstrkycaeiqvmhxpw';."
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let layout = stats.corpus.layout_with(layout_keys);

        layout
            .trigrams()
            .take(35)
            .zip((0..30usize).combinations_with_replacement(3))
            .map(|(c, i)| {
                (
                    stats.corpus.decode([c.0, c.1, c.2]).collect::<String>(),
                    &stats.keyboard_trigrams[(i[0], i[1], i[2])],
                    stats.corpus[c],
                )
            })
            .for_each(|(chars, t, f)| println!("\"{chars}\": {f}, {t}"));
    }
}
