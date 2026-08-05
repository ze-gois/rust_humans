/// Whether the note would be drawn as a black piano key.
pub fn is_black_key(semitone_from_a4: i32) -> bool {
    matches!(
        crate::audible::music::note::pitch_class_from_semitone(semitone_from_a4),
        1 | 3 | 6 | 8 | 10
    )
}
