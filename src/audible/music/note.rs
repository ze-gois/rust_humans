//! Musical notes and pitch classes.

use std::fmt;

use super::temperament::{EqualTemperament, Temperament};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PitchClass {
    C = 0,
    Cs = 1,
    D = 2,
    Ds = 3,
    E = 4,
    F = 5,
    Fs = 6,
    G = 7,
    Gs = 8,
    A = 9,
    As = 10,
    B = 11,
}

impl PitchClass {
    pub const ALL: [Self; 12] = [
        Self::C,
        Self::Cs,
        Self::D,
        Self::Ds,
        Self::E,
        Self::F,
        Self::Fs,
        Self::G,
        Self::Gs,
        Self::A,
        Self::As,
        Self::B,
    ];
    pub const fn from_index(index: u8) -> Self {
        Self::ALL[(index % 12) as usize]
    }
    pub const fn index(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::C => "C",
            Self::Cs => "C#",
            Self::D => "D",
            Self::Ds => "D#",
            Self::E => "E",
            Self::F => "F",
            Self::Fs => "F#",
            Self::G => "G",
            Self::Gs => "G#",
            Self::A => "A",
            Self::As => "A#",
            Self::B => "B",
        })
    }
}

/// A chromatic note using scientific pitch notation (C4 is middle C).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Note {
    pub pitch: PitchClass,
    pub octave: i8,
}

impl Note {
    pub const fn new(pitch: PitchClass, octave: i8) -> Self {
        Self { pitch, octave }
    }
    pub const fn a4() -> Self {
        Self::new(PitchClass::A, 4)
    }
    pub fn semitone_from_a4(self) -> i32 {
        (self.octave as i32 - 4) * 12 + self.pitch.index() - 9
    }
    pub fn frequency(self) -> f32 {
        EqualTemperament::STANDARD.frequency(self.semitone_from_a4())
    }
    pub fn transpose(self, semitones: i32) -> Self {
        let absolute = self.octave as i32 * 12 + self.pitch.index() + semitones;
        let octave = absolute.div_euclid(12);
        Self::new(
            PitchClass::from_index(absolute.rem_euclid(12) as u8),
            octave as i8,
        )
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch, self.octave)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a4_is_440_hz() {
        assert!((Note::a4().frequency() - 440.0).abs() < 0.001);
    }
    #[test]
    fn transposition_crosses_octaves() {
        assert_eq!(
            Note::new(PitchClass::B, 3).transpose(1),
            Note::new(PitchClass::C, 4)
        );
    }
}
