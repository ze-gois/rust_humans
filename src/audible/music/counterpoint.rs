//! Small helpers for combining independent voices.

use super::phrase::{Event, Phrase};

/// A set of voices intended to be played at the same time.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Counterpoint {
    voices: Vec<Phrase>,
}

impl Counterpoint {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_voices(voices: impl IntoIterator<Item = Phrase>) -> Self {
        Self {
            voices: voices.into_iter().collect(),
        }
    }
    pub fn add_voice(&mut self, voice: Phrase) {
        self.voices.push(voice);
    }
    pub fn voices(&self) -> &[Phrase] {
        &self.voices
    }
    pub fn total_beats(&self) -> f32 {
        self.voices
            .iter()
            .map(Phrase::total_beats)
            .fold(0.0, f32::max)
    }

    /// Returns events at a shared time grid, padding shorter voices with rests.
    pub fn aligned(&self) -> Vec<Vec<Event>> {
        let longest = self
            .voices
            .iter()
            .map(|v| v.events().len())
            .max()
            .unwrap_or(0);
        (0..longest)
            .map(|i| {
                self.voices
                    .iter()
                    .map(|voice| {
                        voice
                            .events()
                            .get(i)
                            .copied()
                            .unwrap_or(Event::rest(super::rhythm::Duration::QUARTER))
                    })
                    .collect()
            })
            .collect()
    }
}
