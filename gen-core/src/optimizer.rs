use crate::data::Data;
use itertools::Itertools;
use libdof::dofinitions::Finger;
use std::{collections::HashMap, fmt::Write as _, io::Write as _};

use crate::{
    keyboard::Pos, layout::Layout, mapping::Mapping, trigram_types::TrigramTypes, weights::Weights,
};

pub type OptimizerTrigrams = (u32, [u8; 3]);

/// The core of layout generation and optimization.
/// # Examples
/// Initialize a simple `Optimizer`
/// ```
/// use gen_core::prelude::*;
/// use Finger::*;
///
/// #[rustfmt::skip]
/// let fingers = [
///     LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
///     LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP, RP,
///     LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
/// ];
///
/// let keyboard = Keyboard::new(&fingers);
/// let types = TrigramTypes::with_defaults(keyboard);
///
/// let data = Data::load("../data/shai.json").expect("couldn't load data");
///
/// let weights = Weights::load("./weights.toml").expect("Couldn't read weights");
///
/// let optimizer = Optimizer::new(&types, data, weights);
/// ```
/// Create a
#[derive(Clone)]
pub struct Optimizer<'a> {
    types: Box<[&'a str]>,
    freqs: Box<[f32]>,
    swap_list: Box<[(Pos, Pos)]>,
    swap_affected_trigrams: HashMap<(Pos, Pos), Box<[OptimizerTrigrams]>>,
    weights: Box<[f32]>,
    mapping: Mapping,
    len: usize,
}

impl<'a> Optimizer<'a> {
    pub fn new(trigram_types: &'a TrigramTypes<'a>, trigram_freqs: Data, weights: Weights) -> Self {
        let mut trigrams = Vec::with_capacity(trigram_types.keyboard().len().pow(3));
        let mut weight_vec = Vec::new();
        let mut t_write = String::new();

        for i in 0..trigram_types.keyboard().len() {
            for j in 0..trigram_types.keyboard().len() {
                for k in 0..trigram_types.keyboard().len() {
                    let t = trigram_types.get_type([i, j, k]);
                    let f = unsafe { trigram_types.keyboard().get_fs([i, j, k]) };
                    trigrams.push(t.display());

                    let t_weight = weights.get(t.display());
                    let f_weight = weights.get_finger_trigram(f);
                    weight_vec.push(t_weight * f_weight);

                    writeln!(&mut t_write, "[{i}, {j}, {k}]: {}", t.display()).unwrap();
                }
            }
        }

        let mapping = trigram_freqs.inner().keys().flatten().collect::<Mapping>();

        let mut freqs = vec![0.0; mapping.len().pow(3)];

        for ([c1, c2, c3], f) in trigram_freqs.into_inner() {
            let i1 = mapping.get_u(c1) * mapping.len().pow(2);
            let i2 = mapping.get_u(c2) * mapping.len();
            let i3 = mapping.get_u(c3);
            freqs[i1 + i2 + i3] = f;
        }

        let swaps_total = (0..3)
            .map(|_| 0..trigram_types.keyboard().len())
            .multi_cartesian_product()
            .map(|v| [v[0], v[1], v[2]])
            .collect::<Vec<_>>();

        let mut swap_affected_trigrams = HashMap::new();
        let mut swap_list = Vec::new();

        for i in 0..trigram_types.keyboard().len() {
            for j in 0..trigram_types.keyboard().len() {
                swap_list.push((i, j));

                if i == j {
                    swap_affected_trigrams.insert((i, j), Box::default());
                } else {
                    let insert = swaps_total
                        .iter()
                        .filter(|a| a.contains(&i) || a.contains(&j))
                        .copied()
                        .map(|[v1, v2, v3]| {
                            (
                                (v1 * trigram_types.keyboard().len().pow(2)
                                    + v2 * trigram_types.keyboard().len()
                                    + v3) as u32,
                                [v1 as u8, v2 as u8, v3 as u8],
                            )
                        })
                        .collect::<Vec<_>>();

                    swap_affected_trigrams.insert((i, j), insert.into());
                }
            }
        }

        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("./trigrams.txt")
            .unwrap();

        f.write_all(t_write.as_bytes()).unwrap();

        Self {
            types: trigrams.into(),
            freqs: freqs.into(),
            swap_list: swap_list.into(),
            swap_affected_trigrams,
            weights: weight_vec.into(),
            mapping,
            len: trigram_types.keyboard().len(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_freq(&self, [c1, c2, c3]: [char; 3]) -> f32 {
        let (i1, i2, i3) = (
            self.mapping.get_u(c1),
            self.mapping.get_u(c2),
            self.mapping.get_u(c3),
        );

        let i1 = i1 * self.mapping.len().pow(2);
        let i2 = i2 * self.mapping.len();

        self.freqs[i1 + i2 + i3]
    }

    pub fn get_t(&self, [i1, i2, i3]: [Pos; 3]) -> &str {
        let r1 = i1 * self.len().pow(2);
        let r2 = i2 * self.len();

        self.types
            .get(r1 + r2 + i3)
            .unwrap_or_else(|| panic!("[{i1}, {i2}, {i3}] is not a valid set of positions"))
    }

    pub fn get_w(&self, [i1, i2, i3]: [Pos; 3]) -> f32 {
        let r1 = i1 * self.len().pow(2);
        let r2 = i2 * self.len();

        *self
            .weights
            .get(r1 + r2 + i3)
            .unwrap_or_else(|| panic!("[{i1}, {i2}, {i3}] is not a valid set of positions"))
    }

    pub fn get_f(&self, [i1, i2, i3]: [usize; 3]) -> f32 {
        let r1 = i1 * self.mapping.len().pow(2);
        let r2 = i2 * self.mapping.len();

        *self
            .freqs
            .get(r1 + r2 + i3)
            .unwrap_or_else(|| panic!("[{i1}, {i2}, {i3}] is not a valid set of positions"))
    }

    pub fn layout(&self, chars: &[char], fingers: &[Finger]) -> Option<Layout> {
        let keys = chars
            .iter()
            .map(|c| self.mapping.get_u(*c))
            .collect::<Vec<_>>();

        Layout::from_vecs(keys, fingers.into())
    }

    pub fn random_layout(&self, chars: &[char], fingers: &[Finger]) -> Option<Layout> {
        let keys = chars
            .iter()
            .map(|c| self.mapping.get_u(*c))
            .collect::<Vec<_>>();

        Layout::random(keys, fingers.into())
    }

    pub fn affected_trigrams(&self, p1: Pos, p2: Pos) -> &[OptimizerTrigrams] {
        self.swap_affected_trigrams.get(&(p1, p2)).unwrap()
    }

    pub fn calc_trigram_types(&self, layout: &Layout) -> HashMap<&str, f32> {
        let mut res = HashMap::new();

        for i in 0..layout.len() {
            for j in 0..layout.len() {
                for k in 0..layout.len() {
                    let keys = unsafe { layout.kt([i, j, k]) };

                    let ttype = self.get_t([i, j, k]);
                    let freq = self.get_f(keys);
                    res.entry(ttype).and_modify(|f| *f += freq).or_default();
                }
            }
        }
        res
    }

    pub fn calc_score(&self, layout: &Layout) -> f32 {
        let mut res = 0.0;

        for i in 0..layout.len() {
            for j in 0..layout.len() {
                for k in 0..layout.len() {
                    let keys = unsafe { layout.kt([i, j, k]) };

                    let weight = self.get_w([i, j, k]);
                    let freq = self.get_f(keys);

                    res += weight * freq;
                }
            }
        }
        res
    }

    pub fn apply_best_swap_no_cache(&self, layout: &mut Layout) -> bool {
        let mut best_swap = None;
        let mut best_score = self.calc_score(layout);

        for (p1, p2) in self.swap_list.iter().copied() {
            unsafe { layout.swap(p1, p2) };

            let new_score = self.calc_score(layout);

            if new_score > best_score {
                best_swap = Some((p1, p2));
                best_score = new_score;
            }

            unsafe { layout.swap(p1, p2) };
        }

        match best_swap {
            Some((p1, p2)) => {
                unsafe { layout.swap(p1, p2) };
                true
            }
            None => false,
        }
    }

    pub fn optimize_no_cache(&self, layout: &mut Layout) {
        while self.apply_best_swap_no_cache(layout) {
            println!("Apply swap no cache")
        }
    }

    pub fn generate_no_cache(&self, chars: &[char], fingers: &[Finger]) -> Option<Layout> {
        let mut layout = self.random_layout(chars, fingers)?;

        self.optimize_no_cache(&mut layout);

        Some(layout)
    }

    pub fn apply_best_swap(&self, layout: &mut Layout, cache: &mut Cache) -> bool {
        let mut best_swap = None;
        let mut best_score = cache.total;

        for (p1, p2) in self.swap_list.iter().copied() {
            let original_score = cache.total;

            unsafe { layout.swap(p1, p2) };

            for (i, [u1, u2, u3]) in self.affected_trigrams(p1, p2) {
                cache.total -= cache.main[*i as usize];

                let weight = unsafe { *self.weights.get_unchecked(*i as usize) };

                let [k1, k2, k3] = unsafe { layout.kt([*u1 as usize, *u2 as usize, *u3 as usize]) };
                let freq_i = k1 * self.mapping.len().pow(2) + k2 * self.mapping.len() + k3;
                let freq = self.freqs[freq_i];

                cache.total += weight * freq;
            }

            if cache.total > best_score + self.freqs.len() as f32 * f32::EPSILON {
                best_swap = Some((p1, p2));
                best_score = cache.total;
            }

            cache.total = original_score;
            unsafe { layout.swap(p1, p2) };
        }

        match best_swap {
            Some((p1, p2)) => {
                unsafe { layout.swap(p1, p2) };

                for (i, [u1, u2, u3]) in self.affected_trigrams(p1, p2) {
                    let weight = *self.weights.get(*i as usize).unwrap();

                    let [k1, k2, k3] =
                        unsafe { layout.kt([*u1 as usize, *u2 as usize, *u3 as usize]) };
                    let freq_i = k1 * self.mapping.len().pow(2) + k2 * self.mapping.len() + k3;
                    let freq = self.freqs[freq_i];

                    cache.main[*i as usize] = weight * freq;
                }

                cache.total = cache.main.iter().sum::<f32>();
                true
            }
            None => false,
        }
    }

    pub fn optimize(&self, layout: &mut Layout) {
        let cache = &mut self.new_cache(layout);

        while self.apply_best_swap(layout, cache) {}
    }

    pub fn generate(&self, chars: &[char], fingers: &[Finger]) -> Option<Layout> {
        let mut layout = self.random_layout(chars, fingers)?;

        self.optimize(&mut layout);

        Some(layout)
    }

    fn new_cache(&self, layout: &Layout) -> Cache {
        let mut buf = Vec::new();

        for i in 0..layout.len() {
            for j in 0..layout.len() {
                for k in 0..layout.len() {
                    let keys = unsafe { layout.kt([i, j, k]) };

                    let weight = self.get_w([i, j, k]);
                    let freq = self.get_f(keys);

                    buf.push(weight * freq);
                }
            }
        }

        Cache::new(buf)
    }

    pub fn layout_to_str(&self, layout: &Layout) -> String {
        use libdof::dofinitions::Finger::*;

        let mut last_finger = RP;
        let mut res = String::new();

        layout
            .keys()
            .iter()
            .zip(layout.fingers())
            .for_each(|(k, f)| {
                match (last_finger, f) {
                    (RP, LP) => res.push('\n'),
                    (LI, RI) => res.push_str("  "),
                    _ => res.push(' '),
                };
                res.push(self.mapping.get_c(*k));
                last_finger = *f;
            });

        res
    }

    pub fn print_layout(&self, name: &str, layout: &Layout) {
        let layout_str = self.layout_to_str(layout);
        let score = self.calc_score(layout);
        let freqs = self.calc_trigram_types(layout);

        println!("{name}: {score}{layout_str}\n");

        for (ttype, freq) in freqs {
            println!("{:<15}{:>6}%", format!("{ttype}:"), format!("{freq:.3}"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cache {
    main: Box<[f32]>,
    total: f32,
}

impl Cache {
    pub fn new(buffer: Vec<f32>) -> Self {
        Cache {
            total: buffer.iter().sum(),
            main: buffer.clone().into(),
        }
    }

    // pub fn swap(&mut self) {
    //     std::mem::swap(&mut self.main, &mut self.other);
    // }

    // pub fn reset(&mut self) {
    //     self.main = self.other.clone()
    // }
}
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_random() {
        use crate::keyboard::Keyboard;
        use libdof::dofinitions::Finger::*;
        use time_this::time;

        #[rustfmt::skip]
        let fingers = [
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
        ];

        let keyboard = Keyboard::new(&fingers);
        let types = TrigramTypes::with_defaults(keyboard);

        let data = Data::load("../data/shai.json").expect("couldn't load read data");

        let weights = Weights::load("./weights.toml").expect("Couldn't read weights");

        let optimizer = Optimizer::new(&types, data, weights);

        let chars = r#"
            r t p w g  f z , a u
            l d c s b  h k i o / e
            n m y v q  ' x j . ;
        "#
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();

        let mut layouts = Vec::new();

        for _ in 0..50 {
            let layout = time!(optimizer.generate(&chars, &fingers)).unwrap();
            let score = optimizer.calc_score(&layout);
            layouts.push((layout, score));
        }

        layouts.sort_by(|(_, s1), (_, s2)| s2.partial_cmp(s1).unwrap());

        optimizer.print_layout("best layout in 100 layouts", &layouts[0].0);
    }

    #[test]
    fn affected() {
        use crate::keyboard::Keyboard;
        use libdof::dofinitions::Finger::*;

        #[rustfmt::skip]
        let fingering = [
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
        ];

        let keyboard = Keyboard::new(&fingering);
        let types = TrigramTypes::with_defaults(keyboard);

        let data = Data::load("../data/shai.json").expect("couldn't load read data");

        let weights = Weights::load("./weights.toml").expect("Couldn't read weights");

        let optimizer = Optimizer::new(&types, data, weights);

        for t in optimizer.affected_trigrams(0, 11) {
            println!("{t:?}");
        }
    }

    #[test]
    fn qwerty_optimal() {
        use crate::keyboard::Keyboard;
        use libdof::dofinitions::Finger::*;

        #[rustfmt::skip]
        let fingering = [
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
        ];

        let keyboard = Keyboard::new(&fingering);
        let types = TrigramTypes::with_defaults(keyboard);

        let data = Data::load("../data/shai.json").expect("couldn't load read data");

        let weights = Weights::load("./weights.toml").expect("Couldn't read weights");

        let optimizer = Optimizer::new(&types, data, weights);

        let qwerty_optimal = r#"
            r t p w g  f z , a u
            l d c s b  h k i o / e
            n m y v q  ' x j . ;
        "#
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();

        let qwerty_optimal = optimizer.layout(&qwerty_optimal, &fingering).unwrap();

        optimizer.print_layout("qwerty_optimal", &qwerty_optimal);
    }

    #[test]
    fn score() {
        use crate::keyboard::Keyboard;
        use libdof::dofinitions::Finger::*;
        use time_this::time;

        #[rustfmt::skip]
        let fingering = [
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
        ];

        let keyboard = Keyboard::new(&fingering);
        let types = TrigramTypes::with_defaults(keyboard);

        let data = Data::load("../data/shai.json").expect("couldn't load read data");

        let weights = Weights::load("./weights.toml").expect("Couldn't read weights");

        let optimizer = Optimizer::new(&types, data, weights);

        let stronk = r#"
            f d l b v  j g o u ,
            s t r n k  y m a e i -
            z q x h p  w c ' ; .
        "#
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();

        let qwerty = r#"
            q w e r t  y u i o p
            a s d f g  h j k l ; '
            z x c v b  n m , . /
        "#
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();

        let mut stronk = optimizer.layout(&stronk, &fingering).unwrap();
        let mut qwerty = optimizer.layout(&qwerty, &fingering).unwrap();

        let stronk_score = optimizer.calc_score(&stronk);
        let qwerty_score = optimizer.calc_score(&qwerty);

        println!("stronk score is {stronk_score}\nqwerty score is {qwerty_score}");

        let stronk_cache = &mut optimizer.new_cache(&stronk);
        let qwerty_cache = &mut optimizer.new_cache(&qwerty);

        optimizer.apply_best_swap(&mut stronk, stronk_cache);
        optimizer.apply_best_swap(&mut qwerty, qwerty_cache);

        let stronk_score_swap = optimizer.calc_score(&stronk);
        let qwerty_score_swap = optimizer.calc_score(&qwerty);

        println!("stronk score after swap is {stronk_score_swap}\nqwerty score after swap is {qwerty_score_swap}");

        println!(
            "stronk after swap:\n{}\n\nqwerty after swap:\n{}",
            optimizer.layout_to_str(&stronk),
            optimizer.layout_to_str(&qwerty)
        );

        time!(optimizer.optimize(&mut qwerty));

        optimizer.print_layout("qwerty optimized", &qwerty);

        time!(optimizer.optimize(&mut qwerty));

        optimizer.print_layout("qwerty optimized", &qwerty);
    }

    #[test]
    fn thing() {
        use crate::{keyboard::Keyboard, REPLACEMENT_CHAR};
        use libdof::dofinitions::Finger::*;

        #[rustfmt::skip]
        let fingering = [
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
        ];

        let keyboard = Keyboard::new(&fingering);
        let types = TrigramTypes::with_defaults(keyboard);

        let data = Data::load("../data/shai.json").expect("couldn't load read data");

        let optimizer = Optimizer::new(&types, data, Weights::default());

        let stronk = r#"
            f d l b v  j g o u ,
            s t r n k  y m a e i
            z q x h p  w c ' ; .
        "#
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();

        let layout = optimizer.layout(&stronk, &fingering).unwrap();

        let t1 = ['a', 'b', 'c'];
        let t2 = ['t', 'h', 'e'];
        let t3 = ['d', 'o', 'f'];
        let t4 = [REPLACEMENT_CHAR, 'e', 'e'];

        let fr1 = optimizer.get_freq(t1);
        let fr2 = optimizer.get_freq(t2);
        let fr3 = optimizer.get_freq(t3);
        let fr4 = optimizer.get_freq(t4);

        assert_eq!(fr4, 0.0);

        // let u1 = corpus.get_index(t1);

        println!("{t1:?}: {fr1}");
        println!("{t2:?}: {fr2}");
        println!("{t3:?}: {fr3}");
        println!("{t4:?}: {fr4}");

        let freqs = optimizer.calc_trigram_types(&layout);
        for (ttype, freq) in freqs {
            println!("{:<15}{:>6}%", format!("{ttype}:"), format!("{freq:.3}"))
        }
    }
}
