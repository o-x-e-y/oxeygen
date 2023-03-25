use std::iter;

use fxhash::FxHashMap;
use sliding_window_alt::SlidingWindow;

use crate::{REPEAT_KEY, REPLACEMENT_CHAR, SHIFT_CHAR};

#[derive(Debug)]
pub struct CorpusRefiner {
    // multiple_char_rules: FxHashMap<usize, Vec<(Vec<char>, Vec<char>)>>,
    longest_rule: usize,
    map: FxHashMap<char, Vec<char>>,
    repeat_key: bool,
    raw: bool,
}

pub struct CorpusRefinerBuilder {
    // multiple_char_rules: FxHashMap<usize, Vec<(Vec<char>, Vec<char>)>>,
    longest_rule: usize,
    map: FxHashMap<char, Vec<char>>,
    repeat_key: bool,
}

pub struct RawCorpusRefiner;

impl CorpusRefiner {
    pub fn new() -> CorpusRefinerBuilder {
        CorpusRefinerBuilder {
            // multiple_char_rules: FxHashMap::default(),
            longest_rule: 1,
            map: FxHashMap::default(),
            repeat_key: false,
        }
    }

    pub fn raw() -> RawCorpusRefiner {
        RawCorpusRefiner
    }
}

impl CorpusRefinerBuilder {
    pub fn dead_key<'a>(
        &mut self,
        from_to: impl Iterator<Item = (char, char)>,
        dead_key: char,
    ) -> &mut Self {
        for (from, to) in from_to {
            self.map.insert(from, vec![dead_key, to]);
        }
        self
    }

    pub fn with_uppercase(
        &mut self,
        lower_upper: impl IntoIterator<Item = (char, char)>,
        include_lowercase_versions: bool,
    ) -> &mut Self {
        if include_lowercase_versions {
            for (lower, upper) in lower_upper {
                self.map.insert(lower, vec![lower]);
                self.map.insert(upper, vec![SHIFT_CHAR, lower]);
            }
        } else {
            for (lower, upper) in lower_upper {
                self.map.insert(upper, vec![SHIFT_CHAR, lower]);
            }
        }
        self
    }

    pub fn convert(&mut self, from_to: impl IntoIterator<Item = (char, char)>) -> &mut Self {
        for (from, to) in from_to {
            self.map.insert(from, vec![to]);
        }
        self
    }

    /// Note that uppercase conversions that span multiple characters are skipped (though their
    /// lowercase equivalent is still included).
    pub fn include(
        &mut self,
        included: impl IntoIterator<Item = char>,
        include_uppercase_versions: bool,
    ) -> &mut Self {
        if include_uppercase_versions {
            for inc in included {
                self.map.insert(inc, vec![inc]);

                let upper = inc.to_uppercase().collect::<Vec<_>>();

                if upper.len() == 1 && upper[0] != inc {
                    self.map.insert(upper[0], vec![SHIFT_CHAR, inc]);
                }
            }
        } else {
            for inc in included {
                self.map.insert(inc, vec![inc]);
            }
        }
        self
    }

    pub fn include_space(&mut self) -> &mut Self {
        self.map.insert(' ', vec![' ']);
        self
    }

    pub fn include_ascii_whitespace(&mut self) -> &mut Self {
        self.map.insert(' ', vec![' ']);
        self.map.insert('\n', vec!['\n']);
        self.map.insert('\t', vec!['\t']);
        self
    }

    pub fn include_qwerty_punct_casings(&mut self) -> &mut Self {
        self.with_uppercase(
            [
                ('`', '~'),
                ('1', '!'),
                ('2', '@'),
                ('3', '#'),
                ('4', '$'),
                ('5', '%'),
                ('6', '^'),
                ('7', '&'),
                ('8', '*'),
                ('9', '('),
                ('0', ')'),
                ('[', '{'),
                (']', '}'),
                ('/', '?'),
                ('=', '+'),
                ('-', '_'),
                ('\\', '|'),
                ('\'', '"'),
                (',', '<'),
                ('.', '>'),
                (';', ':'),
            ],
            true,
        )
    }

    pub fn normalize_miscellaneous_punct(&mut self) -> &mut Self {
        self.convert([
            ('´', '\''),
            ('‘', '\''),
            ('’', '\''),
            ('÷', '/'),
            ('‐', '-'),
            ('–', '-'),
            ('—', '-'),
        ])
        .with_uppercase([('«', '\''), ('»', '\''), ('“', '\''), ('”', '\'')], false)
    }

    // pub fn multiple_char_rule(&mut self, from: &str, to: &str) -> &mut Self {
    //     self.longest_rule = self.longest_rule.max(from.len());

    //     self.multiple_char_rules
    //         .entry(from.len())
    //         .and_modify(|e| e.push((from.chars().collect(), to.chars().collect())))
    //         .or_default();
    //     self
    // }

    pub fn repeat_key(&mut self, enable: bool) -> &mut Self {
        self.repeat_key = enable;
        self.longest_rule = self.longest_rule.max(2);
        self
    }

    pub fn build(&mut self) -> CorpusRefiner {
        CorpusRefiner {
            // multiple_char_rules: self.multiple_char_rules,
            longest_rule: self.longest_rule,
            map: std::mem::take(&mut self.map),
            repeat_key: self.repeat_key,
            raw: false,
        }
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct CorpusRefinerIterator<'a, I> {
    refiner: &'a CorpusRefiner,
    iter: iter::Chain<I, iter::Once<char>>,
    window: SlidingWindow<char>,
}

impl<'a, I> CorpusRefinerIterator<'a, I>
where
    I: Iterator<Item = char>,
{
    fn new(iter: I, refiner: &'a CorpusRefiner) -> CorpusRefinerIterator<'a, I> {
        let window = SlidingWindow::new(refiner.longest_rule, REPLACEMENT_CHAR);
        let iter = iter.chain(iter::once(REPLACEMENT_CHAR));

        CorpusRefinerIterator {
            refiner,
            iter,
            window,
        }
    }
}

impl<'a, I> Iterator for CorpusRefinerIterator<'a, I>
where
    I: Iterator<Item = char>,
{
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.iter.next()?;
        if self.refiner.raw {
            return Some(vec![c])
        }

        self.window.push(c);

        if self.refiner.repeat_key && self.window[0] == self.window[1] {
            Some(vec![REPEAT_KEY])
        } else if let Some(to) = self.refiner.map.get(&c) {
            Some(to.clone())
        } else {
            Some(vec![REPLACEMENT_CHAR])
        }
    }
}

pub trait RefineCorpus: Iterator {
    fn refine(self, refiner: &CorpusRefiner) -> CorpusRefinerIterator<'_, Self>
    where
        Self: Iterator<Item = char>,
        Self: Sized,
    {
        CorpusRefinerIterator::new(self, refiner)
    }
}

impl<I: Iterator> RefineCorpus for I where I: Iterator<Item = char> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refiner_test() {
        let corpus = "ABBcde abcd";

        let refiner = CorpusRefiner::new()
            .include("bcd".chars(), true)
            .convert([('A', '0')])
            .include_ascii_whitespace()
            .repeat_key(true)
            .build();

        let translation = corpus
            .chars()
            .refine(&refiner)
            .flatten()
            .collect::<String>();

        println!("{translation}");
    }
}
