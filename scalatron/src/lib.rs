use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

use anyhow::anyhow;
use serde::Deserialize;
use serde_with::DeserializeFromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteName {
    /// with octaves as in MIDI (CDEFGAB) order
    pub const fn semitones_ionian(&self) -> i32 {
        match self {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        }
    }
    /// with the octave being alphabetical (ABCDEFG) order
    pub const fn semitones_aeolian(&self) -> i32 {
        match self {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => -3,
            NoteName::B => -1,
        }
    }
}

impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, DeserializeFromStr)]
pub struct Note {
    pub name: NoteName,
    pub accidental: i32, // -flat, +sharp
}

impl Note {
    /// How many semitones off from C we are, according to Ionian (CDEFGAB) octave-ing.
    pub fn semitones_ionian(&self) -> i32 {
        self.name.semitones_ionian() + self.accidental
    }
    /// Used when you know this note should be *higher* than the other note. (This is standard in modern Western harmony.)
    ///
    /// The answer will ALWAYS be in the range 0..12.
    pub fn semitones_above(&self, other_note: Note) -> i32 {
        let our_semitone = self.semitones_ionian();
        let their_semitone = other_note.semitones_ionian();
        (their_semitone - our_semitone).rem_euclid(12)
    }
    /// Used when you know this note should be *lower* than the other note. (Ancient Greek musical harmony used this, and the curse has endured for thousands of years. Medieval Europeans not understanding that is also why our mode names are all wrong!)
    ///
    /// The answer will ALWAYS be in the range 0..12.
    pub fn semitones_below(&self, other_note: Note) -> i32 {
        12 - self.semitones_above(other_note)
    }
    /// Returns semitones_above or (negated) semitones_below, whichever has the
    /// smaller magnitude. If the magnitude is 6, return -6.
    pub fn semitone_delta(&self, other_note: Note) -> i32 {
        if self.semitones_above(other_note) >= 6 {
            -self.semitones_below(other_note)
        } else {
            self.semitones_above(other_note)
        }
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.name)?;
        match self.accidental.cmp(&0) {
            Ordering::Less => {
                if self.accidental == -2 {
                    write!(f, "𝄫")?;
                } else {
                    for _ in 0..-self.accidental {
                        write!(f, "♭")?;
                    }
                }
            }
            Ordering::Greater => {
                if self.accidental == 2 {
                    write!(f, "𝄪")?;
                } else {
                    for _ in 0..self.accidental {
                        write!(f, "♯")?;
                    }
                }
            }
            Ordering::Equal => (),
        }
        Ok(())
    }
}

impl FromStr for Note {
    type Err = anyhow::Error; // TODO: make an error enum for this
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let name = match chars.next() {
            Some('C') => NoteName::C,
            Some('D') => NoteName::D,
            Some('E') => NoteName::E,
            Some('F') => NoteName::F,
            Some('G') => NoteName::G,
            Some('A') => NoteName::A,
            Some('B') => NoteName::B,
            _ => return Err(anyhow!("first character of a note must be in CDEFGAB")),
        };
        let mut accidental: i32 = 0;
        let mut is_natural = false;
        for ch in chars {
            if is_natural {
                return Err(anyhow!("if natural is present, no other symbols are valid"));
            }
            let delta: i32 = match ch {
                '♭' | 'b' => -1,
                '𝄫' => -2,
                '♯' | '#' => 1,
                '𝄪' => 2,
                '♮' => {
                    if accidental != 0 {
                        return Err(anyhow!("can’t mix naturals with sharps or flats"));
                    }
                    is_natural = true;
                    continue;
                }
                _ => {
                    return Err(anyhow!(
                        "the characters after the note name must be sharps XOR flats. (will accept “#” or “b” as well)"
                    ));
                }
            };
            if accidental != 0 && delta.signum() != accidental.signum() {
                return Err(anyhow!(
                    "can’t mix sharps and flats in one note, you philistine"
                ));
            }
            accidental += delta;
        }
        Ok(Note { name, accidental })
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScalePolarity {
    Descending,
    Ascending,
}

impl ScalePolarity {
    /// Returns `Some(Ascending)` if `n > `0, `Some(Descending)` if `n < 0`,
    /// or `None` if `n == 0`.
    pub fn from_sign(n: i32) -> Option<Self> {
        match n.cmp(&0) {
            Ordering::Less => Some(Self::Descending),
            Ordering::Equal => None,
            Ordering::Greater => Some(Self::Ascending),
        }
    }
}

#[derive(Debug, Clone, Hash, Deserialize)]
pub struct Scale {
    pub names: Vec<String>,
    pub notes: Vec<Note>,
}

#[derive(Debug, Error)]
pub enum ScalePolarityDeterminationError {
    #[error("tried to get polarity of a scale with fewer than two notes")]
    NotEnoughNotes,
    #[error("scale started with tritone ({first_note}, {second_note})")]
    FirstPairWasTritone { first_note: Note, second_note: Note },
    #[error("scale seemed to reverse polarity mid-neutron flow (starting with {first_note}, {second_note})")]
    NeutronFlowReversed { first_note: Note, second_note: Note },
    #[error("consecutive notes {first_note} and {second_note} had the same name")]
    ConsecutiveNotesHadSameName { first_note: Note, second_note: Note },
    #[error("consecutive notes {first_note} and {second_note} had the same pitch")]
    ConsecutiveNotesHadSamePitch { first_note: Note, second_note: Note },
}

impl Scale {
    /**
    Returns whether the scale appears to ascend or descend.

    For a scale to be considered ascending or descending:

    - It must contain at least two notes
    - Its **first two** notes must not be **exactly** a tritone (six
      semitones) apart (e.g. C, F♯)
    - Two consecutive notes **after** the first two must not be **more than**
      six semitones apart (e.g. C, D, B)
    - Two consecutive notes must not have the same name (e.g. C, C♯)
    - Two consecutive notes must not have the same pitch (e.g. C♯, D♭)
    */
    pub fn get_polarity(&self) -> Result<ScalePolarity, ScalePolarityDeterminationError> {
        use ScalePolarityDeterminationError::*;
        let mut first_delta: Option<i32> = None;
        for pair in self.notes.windows(2) {
            let first_note = pair[0];
            let second_note = pair[1];
            if first_note.name == second_note.name {
                return Err(ConsecutiveNotesHadSameName {
                    first_note,
                    second_note,
                });
            }
            let delta = second_note.semitone_delta(first_note);
            if delta == 0 {
                return Err(ConsecutiveNotesHadSamePitch {
                    first_note,
                    second_note,
                });
            }
            debug_assert!(delta.abs() <= 6);
            if let Some(first_delta) = first_delta {
                if delta.abs() == 6 {
                    // just assume this pair is going the right way
                    continue;
                }
                if delta.signum() != first_delta.signum() {
                    return Err(NeutronFlowReversed {
                        first_note,
                        second_note,
                    });
                }
            } else {
                if delta.abs() == 6 {
                    return Err(FirstPairWasTritone {
                        first_note,
                        second_note,
                    });
                }
                first_delta = Some(delta);
            }
        }
        let Some(first_delta) = first_delta else {
            return Err(NotEnoughNotes);
        };
        Ok(ScalePolarity::from_sign(first_delta).unwrap())
    }
}
