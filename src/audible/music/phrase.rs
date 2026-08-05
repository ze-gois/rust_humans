//! Ordered musical phrases.

use super::{note::Note, rythm::Duration};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event {
    pub note: Option<Note>,
    pub duration: Duration,
}

impl Event {
    pub const fn note(note: Note, duration: Duration) -> Self {
        Self {
            note: Some(note),
            duration,
        }
    }
    pub const fn rest(duration: Duration) -> Self {
        Self {
            note: None,
            duration,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Phrase {
    events: Vec<Event>,
}

impl Phrase {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_events(events: impl IntoIterator<Item = Event>) -> Self {
        Self {
            events: events.into_iter().collect(),
        }
    }
    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }
    pub fn events(&self) -> &[Event] {
        &self.events
    }
    pub fn total_beats(&self) -> f32 {
        self.events.iter().map(|e| e.duration.beats()).sum()
    }
    pub fn transpose(&self, semitones: i32) -> Self {
        Self::from_events(self.events.iter().map(|e| Event {
            note: e.note.map(|n| n.transpose(semitones)),
            duration: e.duration,
        }))
    }
}
