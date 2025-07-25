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

pub const OCTAVE_SIZE: i32 = 12;

impl NoteName {
    /// Returns the next note letter in the sequence, wrapping from G to A.
    pub const fn next_letter(&self) -> NoteName {
        match self {
            Self::C => Self::D,
            Self::D => Self::E,
            Self::E => Self::F,
            Self::F => Self::G,
            Self::G => Self::A,
            Self::A => Self::B,
            Self::B => Self::C,
        }
    }
    /// Returns the previous note letter in the sequence, wrapping from A to G.
    pub const fn previous_letter(&self) -> NoteName {
        match self {
            Self::C => Self::B,
            Self::D => Self::C,
            Self::E => Self::D,
            Self::F => Self::E,
            Self::G => Self::F,
            Self::A => Self::G,
            Self::B => Self::A,
        }
    }
    /// (with octaves as in MIDI (CDEFGAB) order)
    pub const fn semitones_ionian(&self) -> i32 {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        }
    }
    /// (with the octave being alphabetical (ABCDEFG) order)
    pub const fn semitones_aeolian(&self) -> i32 {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => -3,
            Self::B => -1,
        }
    }
    pub fn adjacent_letter(&self, polarity: ScalePolarity) -> NoteName {
        if polarity == ScalePolarity::Ascending {
            self.next_letter()
        } else {
            self.previous_letter()
        }
    }
}

impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[macro_export]
macro_rules! note {
    ($name:ident bb) => {
        Note {
            name: NoteName::$name,
            accidental: -2,
        }
    };
    ($name:ident b) => {
        Note {
            name: NoteName::$name,
            accidental: -1,
        }
    };
    ($name:ident) => {
        Note {
            name: NoteName::$name,
            accidental: 0,
        }
    };
    ($name:ident #) => {
        Note {
            name: NoteName::$name,
            accidental: 1,
        }
    };
    ($name:ident ##) => {
        Note {
            name: NoteName::$name,
            accidental: 2,
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, DeserializeFromStr)]
pub struct Note {
    pub name: NoteName,
    pub accidental: i32, // -flat, +sharp
}

#[derive(Debug, Error, PartialEq, Eq, Hash)]
pub enum NoteParseError {
    #[error("note name must be one of C, D, E, F, G, A, B")]
    InvalidName,
    #[error("natural cannot be mixed with sharps or flats")]
    NaturalAccidental,
    #[error("cannot use sharps and flats on the same note")]
    AccidentalMix,
    #[error("accidentals must be one or more of ♭, b, ♯, or #, or exactly one of ♮, 𝄫, or 𝄪")]
    UnknownAccidental,
}

impl Note {
    /// How many semitones off from C we are, according to Ionian (CDEFGAB) octave-ing.
    pub fn semitones_ionian(&self) -> i32 {
        self.name.semitones_ionian() + self.accidental
    }
    /// Used when you know this note should be *higher* than the other note.
    /// (This is standard in modern Western harmony.)
    ///
    /// The answer will ALWAYS be in the range 0..n, where n is the number of
    /// subdivisions of the octave (currently, only 12 is supported).
    pub fn semitones_above(&self, other_note: Note) -> i32 {
        let our_semitone = self.semitones_ionian();
        let their_semitone = other_note.semitones_ionian();
        (our_semitone - their_semitone).rem_euclid(12)
    }
    /// Used when you know this note should be *lower* than the other note.
    /// (Ancient Greek musical harmony used this, and the curse has endured for
    /// thousands of years. Medieval Europeans not understanding that is also
    /// why our mode names are all wrong!)
    ///
    /// The answer will ALWAYS be in the range 0..n, where n is the number of
    /// subdivisions of the octave (currently, only 12 is supported).
    pub fn semitones_below(&self, other_note: Note) -> i32 {
        other_note.semitones_above(*self)
    }
    /// Returns how many steps you must increment/decrement `self` to reach the
    /// pitch of `other_note`. Steps sharp are positive, steps flat are
    /// negative.
    ///
    /// How we return our result depends on `known_polarity`:
    /// - `None`: Returns whichever direction needs the fewest steps. In the
    ///    event of a tie, returns the negative value.
    /// - `Some(ScalePolarity::Ascending)`: Always returns sharps (i.e., as
    ///    if you called `semitones_below`)
    /// - `Some(ScalePolarity::Descending)`: Always returns flats (i.e., as
    ///    if you called `-semitones_above`)
    pub fn semitone_delta(
        &self,
        other_note: Note,
        known_polarity: Option<ScalePolarity>,
    ) -> i32 {
        if let Some(polarity) = known_polarity {
            if polarity == ScalePolarity::Ascending {
                self.semitones_below(other_note)
            } else {
                -self.semitones_above(other_note)
            }
        } else if self.semitones_above(other_note) > OCTAVE_SIZE / 2 {
            self.semitones_below(other_note)
        } else {
            -self.semitones_above(other_note)
        }
    }
    /// Returns another `Note` that is equivalent in *pitch* to this `Note`,
    /// but uses the next letter. (e.g. `G♯.same_pitch_next_letter() == A♭`)
    pub fn same_pitch_next_letter(&self) -> Note {
        let next_letter = self.name.next_letter();
        let accidental = self.accidental
            - Note {
                name: self.name,
                accidental: 0,
            }
            .semitones_below(Note {
                name: next_letter,
                accidental: 0,
            });
        Note {
            name: next_letter,
            accidental,
        }
    }
    /// Returns another `Note` that is equivalent in *pitch* to this `Note`,
    /// but uses the previous letter. (e.g. `A♭.same_pitch_previous_letter() == G♯`)
    pub fn same_pitch_previous_letter(&self) -> Note {
        let previous_letter = self.name.previous_letter();
        let accidental = self.accidental
            + Note {
                name: self.name,
                accidental: 0,
            }
            .semitones_above(Note {
                name: previous_letter,
                accidental: 0,
            });
        Note {
            name: previous_letter,
            accidental,
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
            _ => {
                return Err(anyhow!(
                    "first character of a note must be in CDEFGAB"
                ))
            }
        };
        let mut accidental: i32 = 0;
        let mut is_natural = false;
        for ch in chars {
            if is_natural {
                return Err(anyhow!(
                    "if natural is present, no other symbols are valid"
                ));
            }
            let delta: i32 = match ch {
                '♭' | 'b' => -1,
                '𝄫' => -2,
                '♯' | '#' => 1,
                '𝄪' => 2,
                '♮' => {
                    if accidental != 0 {
                        return Err(anyhow!(
                            "can’t mix naturals with sharps or flats"
                        ));
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
    pub notes: Option<Vec<Note>>,
    pub intervals: Option<Vec<i32>>,
}

#[derive(Debug, Error, PartialEq, Eq, Hash)]
pub enum GetPolarityError {
    #[error("tried to get polarity of a scale with fewer than two notes")]
    NotEnoughNotes,
    #[error("scale started with tritone ({first_note}, {second_note})")]
    FirstPairWasTritone { first_note: Note, second_note: Note },
    #[error("scale seemed to reverse polarity mid-neutron flow (starting with {first_note}, {second_note})")]
    NeutronFlowReversed { first_note: Note, second_note: Note },
    #[error(
        "consecutive notes {first_note} and {second_note} had the same name"
    )]
    ConsecutiveNotesHadSameName { first_note: Note, second_note: Note },
    #[error(
        "consecutive notes {first_note} and {second_note} had the same pitch"
    )]
    ConsecutiveNotesHadSamePitch { first_note: Note, second_note: Note },
    #[error("interval of zero provided")]
    InvalidInterval,
    #[error("provided intervals are not consistently in the same direction")]
    InconsistentIntervals,
}

#[derive(Debug, Error, PartialEq, Eq, Hash)]
pub enum FillBlanksError {
    #[error("error getting polarity: {inner}")]
    GetPolarityError { inner: GetPolarityError },
    #[error("provided notes ({notes:?}) and intervals ({intervals:?}) do not match at index {index}: delta between notes is {delta}, but provided interval is {provided_interval}")]
    ContradictoryInput {
        notes: Vec<Note>,
        intervals: Vec<i32>,
        index: usize,
        delta: i32,
        provided_interval: i32,
    },
    #[error("called fill_blanks on a scale with neither notes nor intervals")]
    EmptyScale,
    #[error("scale's intervals do not add up to ±OCTAVE_SIZE (currently, OCTAVE_SIZE is always 12)")]
    NonOctaveScale,
    #[error("seven-note scale skipped a letter or used letters out of order or used the same letter more than once")]
    InvalidNoteOrder,
}

impl Scale {
    /// If a scale has exactly seven notes, it must use every note name exactly
    /// once (true). Any other scale can do whatever makes the most sense
    /// (false).
    pub fn must_use_every_letter_once(&self) -> bool {
        self.notes
            .as_ref()
            .map(|x| x.len())
            .unwrap_or(0)
            .max(self.intervals.as_ref().map(|x| x.len()).unwrap_or(0))
            == 7
    }
    /**
    Returns whether a scale appears to ascend or descend. For a scale to be
    considered ascending or descending:

    - It must contain at least two notes
    - Its **first two** notes must not be **exactly** a tritone (six
      semitones in 12-tone equal temperament) apart (e.g. C, F♯)
    - Two consecutive notes **after** the first two must not be **more than**
      a tritone apart (e.g. C, D, B)
    - Two consecutive notes must not have the same pitch (e.g. C♯, D♭)
    - Two consecutive notes must not have the same name (e.g. C, C♯) **unless**
      the scale has more than seven notes (in which case it can't be helped
      without ridiculous accidental trains)

    OR:

    - It must already have intervals
    - Its intervals must all have the same sign
    */
    pub fn get_polarity(&self) -> Result<ScalePolarity, GetPolarityError> {
        use GetPolarityError::*;
        if let Some(intervals) = &self.intervals {
            if !intervals.is_empty() {
                let interval_sign = intervals[0].signum();
                if intervals[1..].iter().any(|x| x.signum() != interval_sign) {
                    return Err(InconsistentIntervals);
                } else {
                    return ScalePolarity::from_sign(interval_sign)
                        .ok_or(InvalidInterval);
                }
            }
        }
        let mut first_delta: Option<i32> = None;
        for pair in self
            .notes
            .as_ref()
            .expect("Tried to call get_polarity on a scale with no note")
            .windows(2)
        {
            let first_note = pair[0];
            let second_note = pair[1];
            if first_note.name == second_note.name
                && self.must_use_every_letter_once()
            {
                return Err(ConsecutiveNotesHadSameName {
                    first_note,
                    second_note,
                });
            }
            // TODO: ensure seven-note scales don't skip letters (e.g., A to C)
            let delta = first_note.semitone_delta(second_note, None);
            if delta == 0 {
                return Err(ConsecutiveNotesHadSamePitch {
                    first_note,
                    second_note,
                });
            } else {
                if delta.abs() == OCTAVE_SIZE / 2 {
                    return Err(FirstPairWasTritone {
                        first_note,
                        second_note,
                    });
                }
            }
            debug_assert!(delta.abs() <= OCTAVE_SIZE / 2);
            if let Some(first_delta) = first_delta {
                if delta.abs() == OCTAVE_SIZE / 2 {
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
                if delta.abs() == OCTAVE_SIZE / 2 {
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
    /// Return the intervals recorded in this `Scale` (see `fill_blanks`)
    pub fn get_intervals(&self) -> Option<&[i32]> {
        self.intervals.as_deref()
    }
    /// Return the `Note`s recorded in this `Scale` (see `fill_blanks`)
    pub fn get_notes(&self) -> Option<&[Note]> {
        self.notes.as_deref()
    }
    /// If there are fewer `Note`s than intervals, synthesize the missing
    /// notes. If there are fewer intervals than `Note`s, synthesize the
    /// missing intervals. If there are no intervals or notes, PANIC.
    // TODO: never panic on bad input, only on bad usage
    pub fn fill_blanks(&mut self) -> Result<(), FillBlanksError> {
        use FillBlanksError::*;
        let (notes, intervals) =
            match (self.notes.as_ref(), self.intervals.as_ref()) {
                (None, None) => {
                    return Err(EmptyScale);
                }
                (notes, intervals) => (
                    notes.map(|x| &x[..]).unwrap_or(&[]),
                    intervals.map(|x| &x[..]).unwrap_or(&[]),
                ),
            };
        let amount_of_intervals_to_check = if notes.len() == intervals.len() {
            notes.len()
        } else {
            intervals.len().min(notes.len().saturating_sub(1))
        };
        let polarity = self
            .get_polarity()
            .map_err(|e| FillBlanksError::GetPolarityError { inner: e })?;
        #[allow(clippy::comparison_chain)]
        if notes.len() > intervals.len() {
            // TODO: break into function
            let mut new_intervals = Vec::new();
            new_intervals.extend_from_slice(intervals);
            match polarity {
                ScalePolarity::Descending => {
                    for pair in notes.windows(2).skip(intervals.len()) {
                        new_intervals.push(-pair[1].semitones_below(pair[0]))
                    }
                    new_intervals.push(
                        -notes
                            .first()
                            .unwrap()
                            .semitones_below(*notes.last().unwrap()),
                    );
                }
                ScalePolarity::Ascending => {
                    for pair in notes.windows(2).skip(intervals.len()) {
                        new_intervals.push(pair[1].semitones_above(pair[0]))
                    }
                    new_intervals.push(
                        notes
                            .first()
                            .unwrap()
                            .semitones_above(*notes.last().unwrap()),
                    );
                }
            }
            if new_intervals.iter().copied().sum::<i32>().abs() != OCTAVE_SIZE
            {
                return Err(NonOctaveScale);
            }
            self.intervals = Some(new_intervals);
        } else if notes.len() < intervals.len() {
            // TODO: break into function
            // the rest of the <del>owl (◎▼◎)</del> [notes (♪♫)]
            let mut new_notes = Vec::new();
            new_notes.extend_from_slice(notes);
            let try_reusing_letter_consecutively =
                !self.must_use_every_letter_once();
            if notes.is_empty() {
                new_notes.push(note!(C));
            }
            while new_notes.len() < intervals.len() {
                let i = new_notes.len() - 1;
                let prev_note = new_notes[i];
                let prev_interval = intervals[i];
                let mut new_note = Note {
                    name: prev_note.name,
                    accidental: prev_note.accidental + prev_interval,
                };
                if !try_reusing_letter_consecutively {
                    if polarity == ScalePolarity::Ascending {
                        new_note = new_note.same_pitch_next_letter();
                    } else {
                        new_note = new_note.same_pitch_previous_letter();
                    }
                } else {
                    match polarity {
                        ScalePolarity::Ascending => {
                            while new_note.accidental >= 2
                                || (new_note.accidental == 1
                                    && new_note
                                        .same_pitch_next_letter()
                                        .accidental
                                        == 0)
                            {
                                new_note = new_note.same_pitch_next_letter();
                            }
                        }
                        ScalePolarity::Descending => {
                            while new_note.accidental <= -2
                                || (new_note.accidental == -1
                                    && new_note
                                        .same_pitch_previous_letter()
                                        .accidental
                                        == 0)
                                || (new_note.accidental == -1
                                    && new_note
                                        .same_pitch_previous_letter()
                                        .accidental
                                        == 1)
                            {
                                new_note =
                                    new_note.same_pitch_previous_letter();
                            }
                        }
                    }
                }
                new_notes.push(new_note);
            }
            self.notes = Some(new_notes);
        }
        // From this point forward, we're only checking.
        let notes = self.notes.as_ref().unwrap();
        let intervals = self.intervals.as_ref().unwrap();
        for i in 0..amount_of_intervals_to_check {
            let delta = notes[i]
                .semitone_delta(notes[(i + 1) % notes.len()], Some(polarity));
            if delta != intervals[i] {
                return Err(ContradictoryInput {
                    notes: notes.to_vec(),
                    intervals: intervals.to_vec(),
                    index: i,
                    delta,
                    provided_interval: intervals[i],
                });
            };
        }
        if notes.len() == 7 {
            for i in 0..7 {
                if notes[i].name.adjacent_letter(polarity)
                    != notes[(i + 1) % 7].name
                {
                    return Err(InvalidNoteOrder);
                }
            }
        }
        // <-- check goes here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn literal_ab_test() {
        assert_eq!(note!(A).semitones_below(note!(B)), 2);
    }
    #[test]
    fn same_pitch_next_letter() {
        assert_eq!(note!(A).same_pitch_next_letter(), note!(B bb));
        assert_eq!(note!(E).same_pitch_next_letter(), note!(F b));
        assert_eq!(
            Note {
                name: NoteName::G,
                accidental: 10,
            }
            .same_pitch_next_letter(),
            Note {
                name: NoteName::A,
                accidental: 8
            },
        );
    }
    #[test]
    fn same_pitch_previous_letter() {
        assert_eq!(note!(B bb).same_pitch_previous_letter(), note!(A));
        assert_eq!(note!(F b).same_pitch_previous_letter(), note!(E));
        assert_eq!(
            Note {
                name: NoteName::A,
                accidental: 8
            }
            .same_pitch_previous_letter(),
            Note {
                name: NoteName::G,
                accidental: 10,
            },
        );
    }
    #[test]
    fn scale_insanity() {
        // too wide to determine polarity, Jon Pertwee walked off the set
        let mut scale = Scale {
            names: vec!["Biznebian Pathetic Tritone Scale".to_string()],
            notes: Some(vec![note!(C), note!(F #)]),
            intervals: None,
        };
        assert_eq!(
            scale.fill_blanks(),
            Err(FillBlanksError::GetPolarityError {
                inner: GetPolarityError::FirstPairWasTritone {
                    first_note: note!(C),
                    second_note: note!(F #)
                }
            })
        );
    }
    #[test]
    fn scale_polarity() {
        let ascending_scale = Scale {
            names: vec!["Antediluvian Post-skeptical Scale".to_string()],
            notes: Some(vec![
                note!(C),
                note!(D),
                note!(E),
                note!(F),
                note!(G),
                note!(A),
                note!(B),
            ]),
            intervals: None,
        };
        assert_eq!(
            ascending_scale.get_polarity(),
            Ok(ScalePolarity::Ascending)
        );
        let descending_scale = Scale {
            names: vec!["Postdiluvian Pre-skeptical Scale".to_string()],
            notes: Some(vec![
                note!(B),
                note!(A),
                note!(G),
                note!(F),
                note!(E),
                note!(D),
                note!(C),
            ]),
            intervals: None,
        };
        assert_eq!(
            descending_scale.get_polarity(),
            Ok(ScalePolarity::Descending)
        );
    }
    /// Tests that `fill_blanks` works correctly, returning scales with the
    /// same number of intervals as notes.
    #[test]
    fn scale_sanity() {
        struct TestCase {
            name: &'static str,
            starting_notes: &'static [Note],
            starting_intervals: &'static [i32],
            expected_notes: &'static [Note],
            expected_intervals: &'static [i32],
        }
        let tests: &[TestCase] = &[
            TestCase {
                // same number of intervals and notes
                // (must quietly validate and succeed)
                name: "Solronian Ecstatic Augmented <del>Chord</del> [Scale]",
                starting_notes: &[note!(C), note!(E), note!(G #)],
                starting_intervals: &[4, 4, 4],
                expected_notes: &[note!(C), note!(E), note!(G #)],
                expected_intervals: &[4, 4, 4],
            },
            TestCase {
                // more intervals than notes
                // (must add notes to scale)
                name: "Jovian Suicide Scale",
                starting_notes: &[note!(C), note!(D #), note! (F #)],
                starting_intervals: &[3, 3, 3, 3],
                expected_notes: &[note!(C), note!(D #), note!(F #), note!(A)],
                expected_intervals: &[3, 3, 3, 3],
            },
            TestCase {
                // more notes than intervals
                // (must fill in remaining intervals)
                name: "Izshatonic Guilt Scale",
                starting_notes: &[
                    note!(C),
                    note!(D #),
                    note!(F),
                    note!(G #),
                    note!(A #),
                ],
                starting_intervals: &[3, 2, 3],
                expected_notes: &[
                    note!(C),
                    note!(D #),
                    note!(F),
                    note!(G #),
                    note!(A #),
                ],
                expected_intervals: &[3, 2, 3, 2, 2],
            },
        ];
        let mut ok = true;
        for test in tests {
            let mut scale = Scale {
                names: vec![test.name.to_string()],
                notes: Some(test.starting_notes.to_vec()),
                intervals: Some(test.starting_intervals.to_vec()),
            };
            scale.fill_blanks();
            if scale.get_notes().unwrap() != test.expected_notes {
                println!(
                    "fill_blanks did not match expected notes for scale {}",
                    test.name
                );
                ok = false;
            };
            assert_eq!(
                scale.get_intervals().unwrap(),
                test.expected_intervals
            );
            if scale.get_intervals().unwrap() != test.expected_intervals {
                println!(
                    "fill_blanks did not match expected intervals for scale {}",
                    test.name
                );
                ok = false;
            };
            if scale.get_intervals().unwrap().len()
                != scale.get_notes().unwrap().len()
            {
                ok = false;
                println!(
                    "fill_blanks gave scale {} {} notes and {} intervals (values should match)",
                    test.name,
                    scale.get_notes().unwrap().len(),
                    scale.get_intervals().unwrap().len()
                );
            }
        }
        if !ok {
            panic!("one or more test cases failed");
        }
    }

    /// Test that `fill_blanks` correctly returns an error if a scale’s notes
    /// and intervals are both `None`.
    #[test]
    fn scale_emptiness() {
        let mut scale = Scale {
            names: vec!["Ye Absentmindical Forgottene Scale".to_string()],
            notes: None,
            intervals: None,
        };
        assert_eq!(scale.fill_blanks(), Err(FillBlanksError::EmptyScale));
    }
    /// Test that `semitone_delta` returns the correct offsets for several
    /// crucial intervals. Also tests `semitones_above` and `semitones_below`.
    #[test]
    fn semitone_delta_test() {
        const TEST_CASES: &[(Note, Note, i32, i32, i32)] = &[
            (note!(C), note!(D), 10, 2, 2),
            (note!(D), note!(E), 10, 2, 2),
            (note!(E), note!(F), 11, 1, 1),
            (note!(F), note!(G), 10, 2, 2),
            (note!(G), note!(A), 10, 2, 2),
            (note!(A), note!(B), 10, 2, 2),
            (note!(B), note!(C), 11, 1, 1),
            (note!(C), note!(F #), 6, 6, -6),
            (note!(C), note!(G), 5, 7, -5),
        ];
        let mut ok = true;
        for (lhs, rhs, correct_above, correct_below, correct_delta) in
            TEST_CASES
        {
            let our_delta = lhs.semitone_delta(*rhs, None);
            if our_delta != *correct_delta {
                println!("{lhs}.semitone_delta({rhs}) → {our_delta} (WRONG, should be {correct_delta})");
                ok = false;
            } else {
                println!("{lhs}.semitone_delta({rhs}) → {our_delta}");
            }
            let our_above = lhs.semitones_above(*rhs);
            if our_above != *correct_above {
                println!("{lhs}.semitones_above({rhs}) → {our_above} (WRONG, should be {correct_above})");
                ok = false;
            } else {
                println!("{lhs}.semitones_above({rhs}) → {our_above}");
            }
            let our_below = lhs.semitones_below(*rhs);
            if our_below != *correct_below {
                println!("{lhs}.semitones_below({rhs}) → {our_below} (WRONG, should be {correct_below})");
                ok = false;
            } else {
                println!("{lhs}.semitones_below({rhs}) → {our_below}");
            }
            println!()
        }
        if !ok {
            panic!("Some test cases failed!");
        }
    }
    /// Test that `fill_blanks` starts scales on C if no notes are specified.
    #[test]
    fn c_base_test() {
        let mut c_base_test = Scale {
            names: vec!["Indefinite Note Scale".to_string()],
            notes: None,
            intervals: Some(vec![2, 2, 1, 2, 2, 2, 1]),
        };
        c_base_test.fill_blanks();
        assert_eq!(
            c_base_test.notes.unwrap()[0],
            note!(C),
            "Did not assume scale with no notes started on C"
        );
    }
    /// Test that `fill_blanks` does not change the roots of scales with notes
    /// specified.
    #[test]
    fn d_base_test() {
        let mut d_base_test = Scale {
            names: vec!["D(efinite) Note Scale".to_string()],
            notes: Some(vec![note!(D)]),
            intervals: Some(vec![2, 2, 1, 2, 2, 2, 1]),
        };
        d_base_test.fill_blanks();
        assert_eq!(
            d_base_test.notes.unwrap()[0],
            note!(D),
            "Scale with first note defined as D somehow wound up starting somewhere else"
        );
    }
    /// Test that `fill_blanks` ALLOWS scales with more than seven notes to reuse
    /// note names consecutively (e.g., ALLOW A, B♭, B, C, D♭, D, E♭, E, F, G♭,
    /// G, A♭ [i.e., the entire chromatic scale])
    #[test]
    fn long_scale_letter_reuse_test() {
        let mut scale = Scale {
            names: vec!["Chromatic Aromatic Scale".to_string()],
            notes: Some(vec![
                note!(A),
                note!(B b),
                note!(B),
                note!(C),
                note!(D b),
                note!(D),
                note!(E b),
                note!(E),
                note!(F),
                note!(G b),
                note!(G),
                note!(A b),
            ]),
            intervals: Some(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        };
        scale.fill_blanks();
    }
    #[test]
    /// Test that `fill_blanks` DISALLOWS scales with seven notes from reusing
    /// note names consecutively (i.e., DISALLOW A♭, A, A♯, D, E, F, G)
    fn short_scale_letter_reuse_test() {
        let mut scale = Scale {
            names: vec![
                "Partly Chromatic Discombobulatory Wreck Scale".to_string()
            ],
            notes: Some(vec![
                note!(A),
                note!(B b),
                note!(B),
                note!(F),
                note!(G b),
                note!(G),
                note!(A b),
            ]),
            intervals: Some(vec![1, 1, 6, 1, 1, 1, 1]),
        };
        assert_eq!(
            scale.fill_blanks(),
            Err(FillBlanksError::GetPolarityError {
                inner: GetPolarityError::ConsecutiveNotesHadSameName {
                    first_note: note!(B b),
                    second_note: note!(B)
                }
            })
        );
    }

    #[test]
    /// Tests whether `fill_blanks` returns the correct notes for scales with
    /// with descending intervals.
    fn test_descending_intervals() {
        let mut scale = Scale {
            names: vec![
                "Ancient Greek Chromatic Aromatic Erotic Scale".to_string()
            ],
            notes: Some(vec![note!(G #)]),
            intervals: Some(vec![
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            ]),
        };
        let expected_notes = vec![
            note!(G #),
            note!(G),
            note!(F #),
            note!(F),
            note!(E),
            note!(D #),
            note!(D),
            note!(C #),
            note!(C),
            note!(B),
            note!(A #),
            note!(A),
        ];
        scale.fill_blanks();
        assert_eq!(scale.get_notes().unwrap(), expected_notes);
    }
    /// Tests whether `fill_blanks` returns the correct intervals for scales
    /// with descending notes.
    #[test]
    fn test_descending_notes() {
        let mut scale = Scale {
            names: vec![
                "Ancient Greek Chromatic Aromatic Erotic Scale".to_string()
            ],
            notes: Some(vec![
                note!(G #),
                note!(G),
                note!(F #),
                note!(F),
                note!(E),
                note!(D #),
                note!(D),
                note!(C #),
                note!(C),
                note!(B),
                note!(A #),
                note!(A),
            ]),
            intervals: None,
        };
        let expected_intervals =
            vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
        scale.fill_blanks();
        assert_eq!(scale.get_intervals().unwrap(), expected_intervals);
    }

    /// Test whether `get_polarity` correctly determines scale polarity from
    /// only one interval (and no notes).
    ///
    /// Dedicated to Ozzy Osbourne (1948-2025).
    #[test]
    fn polarity_of_darkness() {
        let scale = Scale {
            names: vec!["Scala Tenebrarum Polarium Incompletarum".to_string()],
            notes: None,
            intervals: Some(vec![-6]),
        };
        assert_eq!(scale.get_polarity(), Ok(ScalePolarity::Descending));
        let scale = Scale {
            names: vec!["Scala Lucum Polarium Incompletarum".to_string()],
            notes: None,
            intervals: Some(vec![6]),
        };
        assert_eq!(scale.get_polarity(), Ok(ScalePolarity::Ascending));
        let scale = Scale {
            names: vec!["Scala Neutralium Impossibilium".to_string()],
            notes: None,
            intervals: Some(vec![0]),
        };
        assert!(scale.get_polarity().is_err());
    }

    /// When notes and intervals are both provided in full, we should return an
    /// error if they don’t match. We have no way to determine which one is
    /// correct, so we’ll leave it up to users to correct their input.
    #[test]
    fn test_interval_and_note_match() {
        let mut scale = Scale {
            names: vec!["Roman Catholic Scale".to_string()],
            notes: Some(vec![
                note!(C),
                note!(D #),
                note!(E),
                note!(F #),
                note!(G),
                note!(A #),
                note!(B),
            ]),
            intervals: Some(vec![2, 2, 1, 2, 2, 2, 1]),
        };
        assert_eq!(
            scale.fill_blanks(),
            Err(FillBlanksError::ContradictoryInput {
                notes: vec![
                    note!(C),
                    note!(D #),
                    note!(E),
                    note!(F #),
                    note!(G),
                    note!(A #),
                    note!(B)
                ],
                intervals: vec![2, 2, 1, 2, 2, 2, 1],
                index: 0,
                delta: 3,
                provided_interval: 2
            })
        );
    }

    /// Test that `fill_blanks` does not allow seven-note scales to skip letters.
    #[test]
    fn test_name_skip() {
        let mut scale = Scale {
            // Chromatic Mixolydian tonos
            names: vec!["Χρωμᾰτῐκός μιξο-Λῡδός τόνος".to_string()],
            // these should really be C, D, E#, F#, G, A#, B
            notes: Some(vec![
                note!(C),
                note!(D),
                note!(F),
                note!(E ##),
                note!(G),
                note!(B b),
                note!(A ##),
            ]),
            intervals: None,
        };
        assert_eq!(
            scale.fill_blanks(),
            Err(FillBlanksError::InvalidNoteOrder)
        );
    }

    /// Test that `semitone_delta` returns a value of -n/2, where n is the
    /// number of equal divisions of the octave, for tritones. (Currently, -n/2
    /// will always be -6, but when we introduce support for other tuning
    /// systems, this will need to be updated.)
    #[test]
    fn semitone_delta_tritone_test() {
        assert_eq!(note!(C).semitone_delta(note!(F #), None), -6);
    }

    /// Test that `semitones_above` returns a value of n/2, where n is the
    /// number of equal divisions of the octave, for tritones. (Currently, n/2
    /// will always be 6, but when we introduce support for other tuning
    /// systems, this will need to be updated.)
    #[test]
    fn semitones_above_tritone_test() {
        assert_eq!(note!(F).semitones_above(note!(B)), 6);
    }
}
