use std::{collections::HashMap, path::Path};

use libdof::dofinitions::Finger;
use serde::{Deserialize, Serialize};

use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FingerWeights(#[serde_as(as = "HashMap<DisplayFromStr, _>")] HashMap<Finger, f32>);

impl FingerWeights {
    pub fn get(&self, f: Finger) -> f32 {
        *self.0.get(&f).unwrap_or(&0.0)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Weights {
    #[serde(flatten)]
    weights: HashMap<String, f32>,
    fingers: FingerWeights,
}

impl Weights {
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn get(&self, name: &str) -> f32 {
        *self.weights.get(name).unwrap_or(&0.0)
    }

    pub fn get_finger_trigram(&self, [f1, f2, f3]: [Finger; 3]) -> f32 {
        self.fingers.get(f1) + self.fingers.get(f2) + self.fingers.get(f3)
    }
}

#[test]
fn load() {
    let weights = Weights::load("./weights.toml");

    println!("{weights:#?}");
}
