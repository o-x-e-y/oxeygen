use itertools::Itertools;
use thiserror::Error;

use std::{collections::HashMap, fs::File, path::Path};

use crate::{corpus_refiner::CorpusRefiner, data::Data};

#[derive(Default, Clone)]
pub struct Corpus {
    pub(crate) name: String,
    pub(crate) char_to_index: HashMap<char, usize>,
    pub(crate) chars: Vec<char>,
    pub(crate) trigrams: Vec<u32>,
    pub(crate) total: u64,
}

#[derive(Debug, Error)]
pub enum CorpusError {
    #[error("DataError: {0}")]
    DataError(#[from] crate::data::DataError),

    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
}

impl Corpus {
    /// Load an existing compatible `.json` file that exists at the given path.
    /// regular usage:
    /// ```
    /// # use gen_core::corpus::Corpus;
    /// let corpus = Corpus::load("./data/english.json");
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
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
    /// # fn p() -> Result<(), CorpusError> {
    /// let corpus = Corpus::new("./path/to/specific_file.txt", "name here!")?;
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    p();
    /// # }
    /// ```
    /// Or, with a directory:
    /// ```
    /// # use gen_core::corpus::{Corpus, CorpusError};
    /// # fn p() -> Result<(), CorpusError> {
    /// let corpus = Corpus::new("./text", "english")?;
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #    p();
    /// # }
    /// ```
    /// note that you can use any file extension, as long as the contents are valid utf-8.
    #[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), CorpusError> {
        let data: Data = self.into();
        data.save(path)?;
        Ok(())
    }

    /// Returns the amount of different characters in the corpus. The amount of trigrams is always
    /// equal to `.len().pow(3)`.
    pub fn len(&self) -> usize {
        self.chars.len()
    }
}

impl From<Data> for Corpus {
    fn from(data: Data) -> Self {
        let char_to_index = data
            .trigrams
            .keys()
            .flatten()
            .unique()
            .enumerate()
            .map(|(i, &c)| (c, i))
            .collect::<HashMap<_, _>>();

        let mut trigrams = vec![0; char_to_index.len().pow(3)];
        let mut total = 0;

        for (t, freq) in data.trigrams {
            let [t1, t2, t3] = &t;

            let u1 = char_to_index.get(t1).unwrap_or(&0);
            let u2 = char_to_index.get(t2).unwrap_or(&0) * char_to_index.len();
            let u3 = char_to_index.get(t3).unwrap_or(&0) * char_to_index.len().pow(2);

            trigrams[u1 + u2 + u3] = freq;
            total += freq as u64;
        }

        Self {
            name: data.name,
            char_to_index: char_to_index.clone(),
            chars: char_to_index.into_keys().collect_vec(),
            trigrams,
            total,
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
    type Output = u32;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        assert!(index.0 < self.len() && index.1 < self.len() && index.2 < self.len());

        let i1 = index.0;
        let i2 = index.1 * self.len();
        let i3 = index.2 * self.len().pow(2);

        let index = i1 + i2 + i3;

        &self.trigrams[index]
    }
}
