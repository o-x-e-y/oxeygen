use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, serde_conv};
use thiserror::Error;

use crate::{
    corpus::Corpus,
    corpus_refiner::{CorpusRefiner, CorpusRefinerIterator, RefineCorpus},
    REPLACEMENT_CHAR,
};

#[cfg(not(target_arch = "wasm32"))]
mod exclude_wasm {
    pub use std::{
        fs::{File, OpenOptions},
        io::Write,
        path::Path,
    };

    pub use file_chunker::FileChunker;
    pub use memmap2::{Mmap, MmapOptions};
    pub use rayon::prelude::*;
    pub use serde_json::ser::PrettyFormatter;

    pub const TWO_MB: usize = 2 * 1024 * 1024;
}

#[cfg(not(target_arch = "wasm32"))]
use exclude_wasm::*;

serde_conv!(
    TrigramAsStr,
    [char; 3],
    |trigram: &[char; 3]| String::from_iter(trigram),
    |value: String| -> Result<_, DataError> {
        value
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| DataError::TrigramConversionError)
    }
);

pub type FxIndexMap<K, V> = IndexMap<K, V, fxhash::FxBuildHasher>;

#[serde_as]
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub(crate) struct Data {
    pub(crate) name: String,
    #[serde_as(as = "FxIndexMap<TrigramAsStr, _>")]
    pub(crate) trigrams: FxIndexMap<[char; 3], u32>,
}

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Trigrams should contain exactly 3 characters")]
    TrigramConversionError,

    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to create a file chunker")]
    ChunkerInitError,

    #[error("Failed to create appropriate chunks")]
    ChunkerChunkError,

    #[error("Utf8Error: {0}")]
    UTF8Error(#[from] std::str::Utf8Error),

    #[error("Path must be either a directory or a file")]
    FaultyPathError,

    #[error("JsonError: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Specifying a name for the corpus is required")]
    NamelessDataError,
}

impl FromIterator<char> for Data {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut res = Self::default();
        let mut iter = iter.into_iter().chain("  ".chars());

        if let Some(mut c1) = iter.next() {
            if let Some(mut c2) = iter.next() {
                while let Some(c3) = iter.next() {
                    if c1 == REPLACEMENT_CHAR || c2 == REPLACEMENT_CHAR || c3 == REPLACEMENT_CHAR {
                        continue;
                    }

                    res.trigrams
                        .entry([c1, c2, c3])
                        .and_modify(|e| *e += 1)
                        .or_insert(1);

                    c1 = c2;
                    c2 = c3;
                }
            }
        }

        res
    }
}

impl From<&str> for Data {
    fn from(src: &str) -> Self {
        src.chars().collect()
    }
}

impl<'a, I> From<CorpusRefinerIterator<'a, I>> for Data
where
    I: Iterator<Item = char>,
{
    fn from(iter: CorpusRefinerIterator<'a, I>) -> Self {
        iter.flatten().collect()
    }
}

impl std::ops::Add for Data {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (trigram, freq) in rhs.trigrams.into_iter() {
            self.trigrams
                .entry(trigram)
                .and_modify(|f| *f += freq)
                .or_insert(freq);
        }

        self
    }
}

impl Data {
    pub(crate) fn sorted(mut self) -> Self {
        self.trigrams.sort_by(|t1, f1, t2, f2| {
            let ord = f2.cmp(f1);
            if ord == std::cmp::Ordering::Equal {
                t1.cmp(t2)
            } else {
                ord
            }
        });

        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<Self, DataError> {
        let f = File::open(path)?;
        let mmap = unsafe { Mmap::map(&f)? };
        let data = serde_json::from_slice(&mmap)?;
        Ok(data)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn from_path<P: AsRef<Path>>(
        path: P,
        name: &str,
        refiner: &CorpusRefiner,
    ) -> Result<Self, DataError> {
        if path.as_ref().is_file() {
            let f = std::fs::File::open(path)?;
            Self::from_file(f, name, refiner)
        } else if path.as_ref().is_dir() {
            let mut new = std::fs::read_dir(path)?
                .par_bridge()
                .flatten()
                .filter(|entry| entry.path().is_file())
                .map(|entry| {
                    let f = std::fs::File::open(entry.path())?;
                    Self::from_file(f, name, refiner)
                })
                .flatten()
                .reduce(|| Self::default(), |a, b| a + b)
                .sorted();

            new.name = name.to_string();

            Ok(new)
        } else {
            Err(DataError::FaultyPathError)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn from_file(file: File, name: &str, refiner: &CorpusRefiner) -> Result<Data, DataError> {
        let chunker = FileChunker::new(&file).map_err(|_| DataError::ChunkerInitError)?;

        let file_len = file.metadata()?.len() as usize;
        let chunk_count = (file_len / TWO_MB).clamp(1, num_cpus::get());

        let chunks = chunker
            .chunks(chunk_count, Some(' '))
            .map_err(|_| DataError::ChunkerChunkError)?;

        let mut res = chunks
            .into_par_iter()
            .map(|chunk| std::str::from_utf8(chunk))
            .flatten()
            .map(|s| Data::from_iter(s.chars().refine(refiner).flatten()))
            .reduce(|| Data::default(), |a, b| a + b)
            .sorted();

        res.name = name.into();

        Ok(res)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn save<P: AsRef<Path>>(self, folder: P) -> Result<(), DataError> {
        if self.name.is_empty() {
            return Err(DataError::NamelessDataError);
        }

        std::fs::create_dir_all(&folder)?;

        let path = folder.as_ref().join(&self.name).with_extension("json");

        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        let formatter = PrettyFormatter::with_indent(b"\t");
        let mut ser = serde_json::ser::Serializer::with_formatter(vec![], formatter);
        self.serialize(&mut ser)?;

        f.write(ser.into_inner().as_slice())?;

        Ok(())
    }
}

impl From<Corpus> for Data {
    fn from(corpus: Corpus) -> Self {
        let trigrams = corpus
            .chars
            .into_iter()
            .combinations_with_replacement(3)
            .zip(corpus.trigrams)
            .filter(|(_, freq)| *freq > 0)
            .map(|(v, f)| ([v[0], v[1], v[2]], f))
            .collect();

        Self {
            name: corpus.name,
            trigrams,
        }
    }
}

impl From<&Corpus> for Data {
    fn from(corpus: &Corpus) -> Self {
        let trigrams = corpus
            .chars
            .iter()
            .combinations_with_replacement(3)
            .zip(&corpus.trigrams)
            .filter(|(_, &freq)| freq > 0)
            .map(|(v, f)| ([*v[0], *v[1], *v[2]], *f))
            .collect();

        Self {
            name: corpus.name.clone(),
            trigrams,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {
    use super::*;
    use time_this::time;

    #[test]
    fn load_test() {
        // let folder = "/home/oxey/Repos/oxeylyzer/static/text/bokmal";
        let folder = "../corpora/akl/";

        let refiner = CorpusRefiner::new()
            .include("abcdefghijklmnopqrstuvwxyz".chars(), true)
            .include_qwerty_punct_casings()
            .build();
        // let refiner = CorpusRefiner::raw();

        let data = time!(Data::from_path(folder, "bokmal", &refiner).unwrap());
        time!(data.save("../data/").unwrap());
    }

    #[test]
    pub fn from_json() {
        use serde_json::json;

        let json = json!({
            "name": "test",
            "trigrams": {
                "abc": 0,
                "def": 1,
            }
        });

        let data = Data {
            name: "test".into(),
            trigrams: FxIndexMap::from_iter([(['a', 'b', 'c'], 0), (['d', 'e', 'f'], 1)]),
        };

        assert_eq!(data, serde_json::from_value::<Data>(json.clone()).unwrap());
    }

    #[test]
    pub fn to_json() {
        use serde_json::json;

        let json = json!({
            "name": "test",
            "trigrams": {
                "abc": 0,
                "def": 1,
            }
        });

        let data = Data {
            name: "test".into(),
            trigrams: FxIndexMap::from_iter([(['a', 'b', 'c'], 0), (['d', 'e', 'f'], 1)]),
        };

        assert_eq!(json, serde_json::to_value(&data).unwrap());
    }

    #[test]
    pub fn err() {
        use serde_json::json;

        let json1 = json!({
            "name": "test",
            "trigrams": {
                "abcd": 0,
                "ghi": 1,
            }
        });

        let json2 = json!({
            "name": "test",
            "trigrams": {
                "abc": 0,
                "gh": 1,
            }
        });

        assert!(serde_json::from_value::<Data>(json1).is_err());
        assert!(serde_json::from_value::<Data>(json2).is_err());
    }

    fn trigram(data: &Data, t: &str) -> u32 {
        let t: [char; 3] = t.chars().collect::<Vec<_>>().try_into().unwrap();
        *data.trigrams.get(&t).unwrap_or(&0)
    }

    #[test]
    fn data_from_str() {
        let s = "the will of the people.";

        let data = Data::from(s);

        assert_eq!(trigram(&data, "the"), 2);
        assert_eq!(trigram(&data, "peo"), 1);
        assert_eq!(trigram(&data, "e. "), 1);
        assert_eq!(trigram(&data, "dof"), 0);
    }
}
