//! Reusable musical motifs.

use super::{
    note::Note,
    phrase::{Event, Phrase},
    rhythm::Duration,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Riff(Phrase);

impl Riff {
    pub fn new(phrase: Phrase) -> Self {
        Self(phrase)
    }
    pub fn from_notes(notes: impl IntoIterator<Item = Note>, duration: Duration) -> Self {
        Self::new(Phrase::from_events(
            notes.into_iter().map(|note| Event::note(note, duration)),
        ))
    }
    pub fn phrase(&self) -> &Phrase {
        &self.0
    }
    pub fn repeat(&self, count: usize) -> Phrase {
        Phrase::from_events((0..count).flat_map(|_| self.0.events().iter().copied()))
    }
}
