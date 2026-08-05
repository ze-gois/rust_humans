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

/// Convert normalized playback progress into a melody index.
pub fn active_note_index(note_count: usize, progress: f32) -> usize {
    if note_count == 0 {
        0
    } else {
        ((progress.clamp(0.0, 0.999_999) * note_count as f32).floor() as usize).min(note_count - 1)
    }
}

/// Return a non-empty pitch range suitable for graph scaling.
pub fn note_range(melody: &[i32]) -> (i32, i32) {
    let min_note = melody.iter().copied().min().unwrap_or(-12);
    let max_note = melody.iter().copied().max().unwrap_or(12);

    if min_note == max_note {
        (min_note - 1, max_note + 1)
    } else {
        (min_note, max_note)
    }
}

/// Convert a semitone offset from A4 into a MIDI-style pitch class.
pub fn pitch_class_from_semitone(semitone_from_a4: i32) -> i32 {
    (69 + semitone_from_a4).rem_euclid(12)
}

/// Human-readable pitch-class name using sharps.
pub fn pitch_class_name(pitch_class: i32) -> &'static str {
    const NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    NAMES[pitch_class.rem_euclid(12) as usize]
}

/// Human-readable note name with octave, e.g. `A4` or `C#5`.
pub fn get_name_from_semitone(semitone_from_a4: i32) -> String {
    let midi_note = 69 + semitone_from_a4;
    let name = pitch_class_name(midi_note.rem_euclid(12));
    let octave = midi_note.div_euclid(12) - 1;

    format!("{name}{octave}")
}
