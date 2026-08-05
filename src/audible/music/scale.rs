//! Common diatonic scales.

use super::note::Note;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleKind {
    Major,
    NaturalMinor,
    HarmonicMinor,
    Pentatonic,
}

impl ScaleKind {
    pub const fn intervals(self) -> &'static [i32] {
        match self {
            Self::Major => &[0, 2, 4, 5, 7, 9, 11],
            Self::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
            Self::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
            Self::Pentatonic => &[0, 2, 4, 7, 9],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    pub root: Note,
    pub kind: ScaleKind,
}

impl Scale {
    pub const fn new(root: Note, kind: ScaleKind) -> Self {
        Self { root, kind }
    }
    pub fn notes(&self) -> Vec<Note> {
        self.kind
            .intervals()
            .iter()
            .map(|&i| self.root.transpose(i))
            .collect()
    }
    pub fn contains(&self, note: Note) -> bool {
        self.kind.intervals().iter().any(|&i| {
            self.root.transpose(i).semitone_from_a4().rem_euclid(12)
                == note.semitone_from_a4().rem_euclid(12)
        })
    }
}

pub fn major(root: Note) -> Scale {
    Scale::new(root, ScaleKind::Major)
}
pub fn natural_minor(root: Note) -> Scale {
    Scale::new(root, ScaleKind::NaturalMinor)
}
