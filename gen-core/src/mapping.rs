use std::{collections::hash_map::RandomState, hash::BuildHasher};

use indexmap::IndexMap;

use crate::{REPLACEMENT_CHAR, SHIFT_CHAR};

#[derive(Clone, Debug, Default)]
pub struct Mapping<S = RandomState>(IndexMap<char, usize, S>);

impl Mapping {
    pub fn new() -> Self {
        let mut map = Self::default();

        map.push(REPLACEMENT_CHAR);
        map.push(SHIFT_CHAR);

        map
    }
}

impl<S: BuildHasher> Mapping<S> {
    pub fn push(&mut self, c: char) {
        if !self.0.contains_key(&c) {
            self.0.insert(c, self.len());
        }
    }

    pub fn remove(&mut self, c: char) -> Option<usize> {
        self.0.swap_remove(&c)
    }

    pub fn pop(&mut self) -> Option<(char, usize)> {
        self.0.pop()
    }

    pub fn get_u(&self, c: char) -> usize {
        match self.0.get(&c) {
            Some(c) => *c,
            None => 0,
        }
    }

    pub fn get_c(&self, u: usize) -> char {
        match self.0.get_index(u) {
            Some((c, _)) => *c,
            None => REPLACEMENT_CHAR,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn map_c<'a>(&'a self, s: &'a str) -> impl Iterator<Item = usize> + 'a {
        s.chars().map(|c| self.get_u(c))
    }

    pub fn map_u<'a>(&'a self, u: &'a [usize]) -> impl Iterator<Item = char> + 'a {
        u.iter().map(|u| self.get_c(*u))
    }
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        Self::from_iter(value.chars())
    }
}

impl From<String> for Mapping {
    fn from(value: String) -> Self {
        Self::from_iter(value.chars())
    }
}

impl<const N: usize> From<[char; N]> for Mapping {
    fn from(arr: [char; N]) -> Self {
        Self::from_iter(arr)
    }
}

impl From<&[char]> for Mapping {
    fn from(slice: &[char]) -> Self {
        Self::from_iter(slice)
    }
}

impl FromIterator<char> for Mapping {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut res = Self::new();

        for c in iter {
            res.push(c)
        }

        res
    }
}

impl<'a> FromIterator<&'a char> for Mapping {
    fn from_iter<T: IntoIterator<Item = &'a char>>(iter: T) -> Self {
        let mut res = Self::new();

        for c in iter {
            res.push(*c)
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from() {
        let mapping_s = "abcdefhgijklmnopqrstuvwxyz ";
        let mapping = mapping_s.chars().collect::<Mapping>();

        assert_eq!(mapping_s.len() + 2, mapping.len());

        let s = "this is epic-";
        let u = mapping.map_c(s).collect::<Vec<_>>();
        let c = mapping.map_u(&u).collect::<String>();

        assert_eq!(c, "this is epic�")
    }
}
