//! Time values used by musical phrases.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Duration(pub f32);

impl Duration {
    pub const WHOLE: Self = Self(4.0);
    pub const HALF: Self = Self(2.0);
    pub const QUARTER: Self = Self(1.0);
    pub const EIGHTH: Self = Self(0.5);
    pub const fn beats(self) -> f32 {
        self.0
    }
    pub fn seconds(self, bpm: f32) -> f32 {
        self.0 * 60.0 / bpm.max(1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rhythm {
    pub duration: Duration,
    pub rest: bool,
}

impl Rhythm {
    pub const fn note(duration: Duration) -> Self {
        Self {
            duration,
            rest: false,
        }
    }
    pub const fn rest(duration: Duration) -> Self {
        Self {
            duration,
            rest: true,
        }
    }
}
