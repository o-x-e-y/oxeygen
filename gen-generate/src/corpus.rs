use itertools::Itertools;
use thiserror::Error;

use std::{collections::HashMap, path::Path};

use gen_core::{corpus_refiner::CorpusRefiner, data::Data, layout::Layout};

#[derive(Default, Clone)]
pub struct Corpus {
    name: String,
    char_to_index: HashMap<char, usize>,
    chars: Vec<char>,
    trigrams: Vec<f32>,
}

#[derive(Debug, Error)]
pub enum CorpusError {
    #[error("DataError: {0}")]
    DataError(#[from] gen_core::data::DataError),

    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
}

impl Corpus {
    /// Returns the amount of different characters in the corpus. The amount of trigrams is always
    /// equal to `.len().pow(3)`.
    pub fn len(&self) -> usize {
        self.char_to_index.len()
    }

    /// Encodes a char iterator with the corpus
    pub fn encode<'a>(
        &'a self,
        chars: impl IntoIterator<Item = char> + 'a,
    ) -> impl Iterator<Item = Option<usize>> + 'a {
        chars
            .into_iter()
            .map(|c| {
                self.char_to_index
                    .get(&c)
                    .copied()
            })
    }

    pub fn encode_slice<'a>(
        &'a self,
        chars: impl IntoIterator<Item = &'a char> + 'a,
    ) -> impl Iterator<Item = Option<usize>> + 'a {
        chars
            .into_iter()
            .map(|c| {
                self.char_to_index
                    .get(c)
                    .copied()
            })
    }

    pub fn encode_trigram(&self, trigram: [char; 3]) -> Option<[usize; 3]> {
        if let Some(u1) = self.char_to_index.get(&trigram[0]) {

            if let Some(u2) = self.char_to_index.get(&trigram[1]) {

                if let Some(u3) = self.char_to_index.get(&trigram[2]) {
                    return Some([*u1, *u2, *u3]);
                }
            }
        }

        None
    }

    pub fn decode<'a>(
        &'a self,
        chars: impl IntoIterator<Item = usize> + 'a,
    ) -> impl Iterator<Item = Option<char>> + 'a {
        chars
            .into_iter()
            .map(|c| {
                self.chars
                .get(c)
                .copied()
            })
    }

    pub fn decode_slice<'a>(
        &'a self,
        chars: impl IntoIterator<Item = &'a usize> + 'a,
    ) -> impl Iterator<Item = Option<char>> + 'a {
        chars
            .into_iter()
            .copied()
            .map(|c| {
                self.chars
                .get(c)
                .copied()
            })
    }

    pub fn decode_trigram(&self, trigram: [usize; 3]) -> Option<[char; 3]> {
        if let Some(u1) = self.chars.get(trigram[0]) {

            if let Some(u2) = self.chars.get(trigram[1]) {
                
                if let Some(u3) = self.chars.get(trigram[2]) {
                    return Some([*u1, *u2, *u3]);
                }
            }
        }

        None
    }

    pub fn freq(&self, trigram: [char; 3]) -> Option<f32> {
        if let Some(i) = self.encode_trigram(trigram) {
            Some(self[i])
        } else {
            None
        }
    }

    // pub fn layout(&self, layout: [char; KEY_AMOUNT]) -> Option<Layout> {
    //     let mut keys = [Key::MIN; KEY_AMOUNT];
        
    //     for (i, u) in self.encode(layout).enumerate() {
    //         if let Some(key) = u {
    //             keys[i] = key;
    //         } else {
    //             return None
    //         }
    //     }

    //     Some(Layout::new(keys))
    // }
}

#[cfg(not(target_arch = "wasm32"))]
impl Corpus {
    /// Load an existing compatible `.json` file that exists at the given path.
    /// regular usage:
    /// ```
    /// # use gen_core::corpus::Corpus;
    /// let corpus = Corpus::load("./data/english.json");
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, CorpusError> {
        let data = Data::load(path)?;
        Ok(data.into())
    }

    /// Converts files in the provided path into a corpus object. The path both be a file and a
    /// directory. If the path is a file it will get loaded if it is valid utf-8. If it is a directory,
    /// all files inside (non-recursive) with utf-8 formatting will be loaded. You also specify a name,
    /// which is used when saving the corpus.
    /// ```
    /// # use gen_core::corpus::{Corpus, CorpusError};
    /// # use gen_core::corpus_refiner::CorpusRefiner;
    /// # fn p() -> Result<(), CorpusError> {
    /// let refiner = CorpusRefiner::new().build();
    /// let corpus = Corpus::new("./path/to/specific_file.txt", "name here!", &refiner)?;
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    p();
    /// # }
    /// ```
    /// Or, with a directory:
    /// ```
    /// # use gen_core::corpus::{Corpus, CorpusError};
    /// # use gen_core::corpus_refiner::CorpusRefiner;
    /// # fn p() -> Result<(), CorpusError> {
    /// let refiner = CorpusRefiner::new().build();
    /// let corpus = Corpus::new("./text", "english", &refiner)?;
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    p();
    /// # }
    /// ```
    /// note that you can use any file extension, as long as the contents are valid utf-8.
    pub fn new<P: AsRef<Path>>(
        path: P,
        name: &str,
        refiner: &CorpusRefiner,
    ) -> Result<Self, CorpusError> {
        let data = Data::from_path(path, name, refiner)?;
        Ok(data.into())
    }

    /// Saves a `.json` file at the specified location. The path should be a directory, and the final
    /// file will have the same name as the one specified in the corpus object. Any paths that don't
    /// already exist are automatically created.
    /// ```
    /// # use gen_core::corpus::{Corpus, CorpusError};
    /// # fn p() -> Result<(), CorpusError> {
    /// # let corpus = Corpus::default();
    /// corpus.save("./corpora")?;
    /// Ok(())
    /// # }
    /// # fn main() {
    /// # p();
    /// # }
    /// ```
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), CorpusError> {
        let data: Data = self.into();
        data.save(path)?;
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl Corpus {
    /// Load an existing compatible `.json` file that exists at the given path.
    /// regular usage:
    /// ```
    /// # use gen_core::corpus::Corpus;
    /// let corpus = Corpus::load("./data/english.json");
    /// ```
    pub async fn load(url: &str) -> Result<Self, CorpusError> {
        let data = Data::load(url).await?;
        Ok(data.into())
    }
}

impl From<Corpus> for Data {
    fn from(corpus: Corpus) -> Self {
        let trigrams = corpus
            .chars
            .into_iter()
            .combinations_with_replacement(3)
            .zip(corpus.trigrams)
            .filter(|(_, freq)| *freq > 0.0)
            .map(|(v, f)| ([v[0], v[1], v[2]], f))
            .collect();

        Data::new(trigrams, &corpus.name)
    }
}

impl From<&Corpus> for Data {
    fn from(corpus: &Corpus) -> Self {
        let trigrams = corpus
            .chars
            .iter()
            .combinations_with_replacement(3)
            .zip(&corpus.trigrams)
            .filter(|(_, &freq)| freq > 0.0)
            .map(|(v, f)| ([*v[0], *v[1], *v[2]], *f))
            .collect();

            Data::new(trigrams, &corpus.name)
    }
}

impl From<Data> for Corpus {
    fn from(data: Data) -> Self {
        let chars = data
            .inner()
            .keys()
            .flatten()
            .unique()
            .copied()
            .collect::<Vec<_>>();

        let char_to_index = chars
            .iter()
            .copied()
            .enumerate()
            .map(|(u, c)| (c, u))
            .collect::<HashMap<_, _>>();

        let mut trigrams = vec![0.0; chars.len().pow(3)];

        for (t, freq) in data.inner() {
            let [t1, t2, t3] = &t;

            let u1 = char_to_index.get(t1).unwrap_or(&0) * char_to_index.len().pow(2);
            let u2 = char_to_index.get(t2).unwrap_or(&0) * char_to_index.len();
            let u3 = char_to_index.get(t3).unwrap_or(&0);

            trigrams[u1 + u2 + u3] = *freq;
        }

        Self {
            name: data.name().into(),
            char_to_index: char_to_index.clone(),
            chars,
            trigrams,
        }
    }
}

impl FromIterator<char> for Corpus {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Data::from_iter(iter).into()
    }
}

impl From<&str> for Corpus {
    fn from(s: &str) -> Self {
        Data::from(s).into()
    }
}

impl From<String> for Corpus {
    fn from(s: String) -> Self {
        Data::from(s.as_str()).into()
    }
}

impl core::ops::Index<(usize, usize, usize)> for Corpus {
    type Output = f32;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        assert!(index.0 < self.len() && index.1 < self.len() && index.2 < self.len());

        let i1 = index.0 * self.len().pow(2);
        let i2 = index.1 * self.len();
        let i3 = index.2;

        let index = i1 + i2 + i3;

        &self.trigrams[index]
    }
}

impl core::ops::Index<[usize; 3]> for Corpus {
    type Output = f32;

    fn index(&self, index: [usize; 3]) -> &Self::Output {
        assert!(index[0] < self.len() && index[1] < self.len() && index[2] < self.len());

        let i1 = index[0] * self.len().pow(2);
        let i2 = index[1] * self.len();
        let i3 = index[2];

        let index = i1 + i2 + i3;

        &self.trigrams[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let corpus = Corpus::load("../data/akl.json").expect("this should always exist");

        let start = "abc";
        let encoded = corpus.encode(start.chars()).collect::<Option<Vec<_>>>()
            .expect("could not encode {start}");

        let decoded = corpus
            .decode(encoded.clone().into_iter())
            .flatten()
            .collect::<String>();

        assert_eq!(start, decoded)
    }

    #[test]
    fn get() {
        let corpus = Corpus::load("../data/akl.json").expect("this should always exist");

        let the = corpus.freq(['t', 'h', 'e'])
            .expect("trigram \"the\" not found in corpus");

        assert_eq!(
            corpus
                .trigrams
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap()),
            Some(&the)
        );

        let our = corpus.freq(['o', 'u', 'r']).unwrap();
        let dof = corpus.freq(['d', 'o', 'f']).unwrap();
        let akl = corpus.freq(['a', 'k', 'l']).unwrap();
        let zzz = corpus.freq(['z', 'z', 'z']).unwrap();

        assert!(our > dof);
        assert!(dof > akl);
        assert!(akl > zzz);
    }
}
