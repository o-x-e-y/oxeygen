use crate::keyboard::*;

pub trait KeyboardType {
    const KEY_SIZE: f64 = DEFAULT_KEY_SIZE;

    fn distances() -> PhysicalDistances;

    fn fingerings() -> Fingerings;

    fn keyboard() -> Keyboard {
        Keyboard::custom(Self::distances(), Self::fingerings(), Self::KEY_SIZE)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ansi;

#[derive(Debug, Copy, Clone)]
pub struct Iso;

#[derive(Debug, Copy, Clone)]
pub struct AnsiAngle;

#[derive(Debug, Copy, Clone)]
pub struct IsoAngle;

#[derive(Debug, Copy, Clone)]
pub struct Ortho;

impl KeyboardType for Ansi {
    #[rustfmt::skip]
    fn distances() -> PhysicalDistances {
        PhysicalDistances::custom([
            (0.0, 0.0),  (1.0, 0.0),  (2.0, 0.0),  (3.0, 0.0),  (4.0, 0.0),  (5.0, 0.0),  (6.0, 0.0),  (7.0, 0.0),  (8.0, 0.0),  (9.0, 0.0),
            (0.25, 1.0), (1.25, 1.0), (2.25, 1.0), (3.25, 1.0), (4.25, 1.0), (5.25, 1.0), (6.25, 1.0), (7.25, 1.0), (8.25, 1.0), (9.25, 1.0),
            (0.75, 2.0), (1.75, 2.0), (2.75, 2.0), (3.75, 2.0), (4.75, 2.0), (5.75, 2.0), (6.75, 2.0), (7.75, 2.0), (8.75, 2.0), (9.75, 0.0),
        ])
    }

    fn fingerings() -> Fingerings {
        Fingerings::default()
    }
}

impl KeyboardType for Iso {
    fn distances() -> PhysicalDistances {
        Ansi::distances()
    }

    fn fingerings() -> Fingerings {
        Fingerings::default()
    }
}

impl KeyboardType for AnsiAngle {
    fn distances() -> PhysicalDistances {
        Ansi::distances()
    }

    #[rustfmt::skip]
    fn fingerings() -> Fingerings {
        Fingerings::custom([
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LP, LR, LM, LI, LI,  RI, RI, RM, RR, RP,
            LR, LM, LI, LI, LI,  RI, RI, RM, RR, RP,
        ])
    }
}

impl KeyboardType for IsoAngle {
    #[rustfmt::skip]
    fn distances() -> PhysicalDistances {
        PhysicalDistances::custom([
            (0.0, 0.0),  (1.0, 0.0),  (2.0, 0.0),  (3.0, 0.0),  (4.0, 0.0),  (5.0, 0.0),  (6.0, 0.0),  (7.0, 0.0),  (8.0, 0.0),  (9.0, 0.0),
            (0.25, 1.0), (1.25, 1.0), (2.25, 1.0), (3.25, 1.0), (4.25, 1.0), (5.25, 1.0), (6.25, 1.0), (7.25, 1.0), (8.25, 1.0), (9.25, 1.0),
            (-0.25, 2.0),(0.75, 2.0), (1.75, 2.0), (2.75, 2.0), (3.75, 2.0), (5.75, 2.0), (6.75, 2.0), (7.75, 2.0), (8.75, 2.0), (9.75, 0.0),
        ])
    }

    fn fingerings() -> Fingerings {
        Ansi::fingerings()
    }
}

impl KeyboardType for Ortho {
    const KEY_SIZE: f64 = 18.5; // MBK keycap size + 1mm

    #[rustfmt::skip]
    fn distances() -> PhysicalDistances {
        PhysicalDistances::custom([
            (0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0), (4.0, 0.0),  (5.0, 0.0), (6.0, 0.0), (7.0, 0.0), (8.0, 0.0), (9.0, 0.0),
            (0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (3.0, 1.0), (4.0, 1.0),  (5.0, 1.0), (6.0, 1.0), (7.0, 1.0), (8.0, 1.0), (9.0, 1.0),
            (0.0, 2.0), (1.0, 2.0), (2.0, 2.0), (3.0, 2.0), (4.0, 2.0),  (5.0, 2.0), (6.0, 2.0), (7.0, 2.0), (8.0, 2.0), (9.0, 0.0),
        ])
    }

    fn fingerings() -> Fingerings {
        Ansi::fingerings()
    }
}
