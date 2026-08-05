//! Pitch-to-frequency conversions.

/// A tuning system that maps semitone offsets from A4 to frequencies.
pub trait Temperament {
    fn frequency(&self, semitone_from_a4: i32) -> f32;
}

/// Twelve-tone equal temperament with a configurable A4 reference pitch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EqualTemperament {
    pub a4_hz: f32,
}

impl EqualTemperament {
    pub const STANDARD: Self = Self { a4_hz: 440.0 };

    pub const fn new(a4_hz: f32) -> Self {
        Self { a4_hz }
    }

    pub fn frequency_for_ratio(&self, semitones: f32) -> f32 {
        self.a4_hz * 2.0_f32.powf(semitones / 12.0)
    }
}

impl Default for EqualTemperament {
    fn default() -> Self {
        Self::STANDARD
    }
}

impl Temperament for EqualTemperament {
    fn frequency(&self, semitone_from_a4: i32) -> f32 {
        self.frequency_for_ratio(semitone_from_a4 as f32)
    }
}

/// Frequency in standard equal temperament (A4 = 440 Hz).
pub fn frequency(semitone_from_a4: i32) -> f32 {
    EqualTemperament::STANDARD.frequency(semitone_from_a4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_tuning_is_equal_tempered() {
        assert!((frequency(0) - 440.0).abs() < 0.001);
        assert!((frequency(12) - 880.0).abs() < 0.001);
    }
}
