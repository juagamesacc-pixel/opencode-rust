//! Rust port of `packages/app/src/utils/sound.ts` (opencode v1.18.30).
//!
//! Source 102 lines: `SOUND_OPTIONS`, `SoundID`, `soundSrc`, `playSound`,
//! `playSoundById`. `import.meta.glob` audio assets + `Audio` playback are
//! PROVISIONAL; option ids/labels are verbatim.
//! Original file: `packages/app/src/utils/sound.ts`

#![allow(dead_code)]

/// Mirrors `SOUND_OPTIONS` ids + labels (verbatim, source order).
pub const SOUND_OPTIONS: &[(&str, &str)] = &[
    ("alert-01", "sound.option.alert01"),
    ("alert-02", "sound.option.alert02"),
    ("alert-03", "sound.option.alert03"),
    ("alert-04", "sound.option.alert04"),
    ("alert-05", "sound.option.alert05"),
    ("alert-06", "sound.option.alert06"),
    ("alert-07", "sound.option.alert07"),
    ("alert-08", "sound.option.alert08"),
    ("alert-09", "sound.option.alert09"),
    ("alert-10", "sound.option.alert10"),
    ("bip-bop-01", "sound.option.bipbop01"),
    ("bip-bop-02", "sound.option.bipbop02"),
    ("bip-bop-03", "sound.option.bipbop03"),
    ("bip-bop-04", "sound.option.bipbop04"),
    ("bip-bop-05", "sound.option.bipbop05"),
    ("bip-bop-06", "sound.option.bipbop06"),
    ("bip-bop-07", "sound.option.bipbop07"),
    ("bip-bop-08", "sound.option.bipbop08"),
    ("bip-bop-09", "sound.option.bipbop09"),
    ("bip-bop-10", "sound.option.bipbop10"),
    ("staplebops-01", "sound.option.staplebops01"),
    ("staplebops-02", "sound.option.staplebops02"),
    ("staplebops-03", "sound.option.staplebops03"),
    ("staplebops-04", "sound.option.staplebops04"),
    ("staplebops-05", "sound.option.staplebops05"),
    ("staplebops-06", "sound.option.staplebops06"),
    ("staplebops-07", "sound.option.staplebops07"),
    ("nope-01", "sound.option.nope01"),
    ("nope-02", "sound.option.nope02"),
    ("nope-03", "sound.option.nope03"),
    ("nope-04", "sound.option.nope04"),
    ("nope-05", "sound.option.nope05"),
    ("nope-06", "sound.option.nope06"),
    ("nope-07", "sound.option.nope07"),
    ("nope-08", "sound.option.nope08"),
    ("nope-09", "sound.option.nope09"),
    ("nope-10", "sound.option.nope10"),
    ("nope-11", "sound.option.nope11"),
    ("nope-12", "sound.option.nope12"),
    ("yup-01", "sound.option.yup01"),
    ("yup-02", "sound.option.yup02"),
    ("yup-03", "sound.option.yup03"),
    ("yup-04", "sound.option.yup04"),
    ("yup-05", "sound.option.yup05"),
    ("yup-06", "sound.option.yup06"),
];

// PROVISIONAL: pending audio-asset pipeline — mirrors `packages/app/src/utils/sound.ts`.
/// Mirrors `soundSrc(id)` cache lookup descriptor.
#[derive(Debug, Default)]
pub struct SoundCache {
    pub cached: Vec<String>,
}

impl SoundCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_known(id: &str) -> bool {
        SOUND_OPTIONS.iter().any(|(known, _)| *known == id)
    }

    pub fn update_mark_cached(&mut self, id: &str) {
        if Self::is_known(id) && !self.cached.contains(&id.to_string()) {
            self.cached.push(id.to_string());
        }
    }
}
