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
        (our_semitone - their_semitone).rem_euclid(12)
    }
    /// Used when you know this note should be *lower* than the other note. (Ancient Greek musical harmony used this, and the curse has endured for thousands of years. Medieval Europeans not understanding that is also why our mode names are all wrong!)
    ///
    /// The answer will ALWAYS be in the range 0..12.
    pub fn semitones_below(&self, other_note: Note) -> i32 {
        other_note.semitones_above(*self)
    }
    /// Returns semitones_above or (negated) semitones_below, whichever has the
    /// smaller magnitude. If the magnitude is 6, return -6.
    pub fn semitone_delta(&self, other_note: Note) -> i32 {
        if self.semitones_above(other_note) > 6 {
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
pub enum ScalePolarityDeterminationError {
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

impl Scale {
    pub fn can_have_consecutive_letters(&self) -> bool {
        self.notes
            .as_ref()
            .map(|x| x.len())
            .unwrap_or(0)
            .max(self.intervals.as_ref().map(|x| x.len()).unwrap_or(0))
            <= 7
    }
    /**
    Returns whether the scale appears to ascend or descend. The scale must
    have notes.

    For a scale to be considered ascending or descending:

    - It must contain at least two notes
    - Its **first two** notes must not be **exactly** a tritone (six
      semitones) apart (e.g. C, F♯)
    - Two consecutive notes **after** the first two must not be **more than**
      six semitones apart (e.g. C, D, B)
    - Two consecutive notes must not have the same pitch (e.g. C♯, D♭)
    - Two consecutive notes must not have the same name (e.g. C, C♯) UNLESS you have more than seven notes in your scale (in which case it can't be avoided without ridiculous accidental trains)

    OR:

    - It must already have intervals
    */
    pub fn get_polarity(
        &self,
    ) -> Result<ScalePolarity, ScalePolarityDeterminationError> {
        use ScalePolarityDeterminationError::*;
        if let Some(intervals) = &self.intervals {
            if !intervals.is_empty() {
                // TODO: do a sanity check to make sure they're all in one direction
                return ScalePolarity::from_sign(intervals[0])
                    .ok_or(InvalidInterval);
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
                && !self.can_have_consecutive_letters()
            {
                return Err(ConsecutiveNotesHadSameName {
                    first_note,
                    second_note,
                });
            }
            let delta = first_note.semitone_delta(second_note);
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
    pub fn fill_blanks(&mut self) {
        let (notes, intervals) = match (
            self.notes.as_ref(),
            self.intervals.as_ref(),
        ) {
            (None, None) => {
                panic!("called fill_blanks on a scale with neither notes nor intervals")
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
        for i in 0..amount_of_intervals_to_check {
            let delta = notes[i].semitone_delta(notes[(i + 1) % notes.len()]);
            assert_eq!(
                delta,
                intervals[i],
                "provided intervals do not match provided notes\n\
                notes: {notes:?}\n\
                intervals: {intervals:?}\n\
                at index {i}, delta between notes is {delta}, but provided interval is {}",
                intervals[i],
            );
        }
        #[allow(clippy::comparison_chain)]
        if notes.len() > intervals.len() {
            let mut new_intervals = Vec::new();
            new_intervals.extend_from_slice(intervals);
            let polarity = self.get_polarity().unwrap();
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
            assert_eq!(
                new_intervals.iter().copied().sum::<i32>(),
                12,
                "A scale’s intervals didn’t add up to 12. Something is wrong with the input."
            );
            self.intervals = Some(new_intervals);
        } else if notes.len() < intervals.len() {
            // the rest of the <del>owl (◎▼◎)</del> [notes (♪♫)]
            let mut new_notes = Vec::new();
            new_notes.extend_from_slice(notes);
            let _polarity = self.get_polarity().unwrap();
            // TODO: support descending polarity
            let use_consecutive_letters = self.can_have_consecutive_letters();
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
                if !use_consecutive_letters {
                    new_note = new_note.same_pitch_next_letter();
                }
                while new_note.accidental >= 2
                    || (new_note.accidental == 1
                        && new_note.same_pitch_next_letter().accidental == 0)
                {
                    new_note = new_note.same_pitch_next_letter();
                }
                new_notes.push(new_note);
            }
            self.notes = Some(new_notes);
        }
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
            }
        );
    }
    #[test]
    #[should_panic]
    fn scale_insanity() {
        // too wide to determine polarity, Jon Pertwee walked off the set
        let mut scale = Scale {
            names: vec!["Biznebian Pathetic Tritone Scale".to_string()],
            notes: Some(vec![note!(C), note!(F #)]),
            intervals: Some(vec![6, 6]),
        };
        scale.fill_blanks();
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
    #[test]
    fn scale_sanity() {
        // same number of intervals and notes
        // (must quietly validate and succeed)
        let mut scale = Scale {
            names: vec![
                "Solronian Ecstatic Augmented <del>Chord</del> [Scale]"
                    .to_string(),
            ],
            notes: Some(vec![note!(C), note!(E), note!(G #)]),
            intervals: Some(vec![4, 4, 4]),
        };
        scale.fill_blanks();
        // more intervals than notes (must add notes to scale)
        let mut scale = Scale {
            names: vec!["Jovian Suicide Scale".to_string()],
            notes: Some(vec![note!(C), note!(D #), note! (F #)]),
            intervals: Some(vec![3, 3, 3, 3]),
        };
        scale.fill_blanks();
        assert_eq!(
            scale.get_notes(),
            Some(&[note!(C), note!(D #), note!(F #), note!(A),] as &[Note])
        );
        // more notes than intervals (must fill in remaining intervals)
        let mut scale = Scale {
            names: vec!["Izshatonic Guilt Scale".to_string()],
            notes: Some(vec![
                note!(C),
                note!(D #),
                note!(F),
                note!(G #),
                note!(A #),
            ]),
            intervals: Some(vec![3, 2, 3]),
        };
        scale.fill_blanks();
        assert_eq!(scale.get_intervals(), Some(&[3, 2, 3, 2, 2] as &[i32]));
        // what if no notes?
        let scale = Scale {
            names: vec!["Ye Absentmindical Forgottene Scale".to_string()],
            notes: None,
            intervals: None,
        };
        assert_eq!(scale.get_notes(), None);
        assert_eq!(scale.get_intervals(), None);
        // TODO: Test case, assume C is base if no notes provided
        // TODO: Test that more than seven notes validates with consecutive note names
        // TODO: Test that seven or fewer notes does not validate with consecutive note names
        // TODO: Test case for filling in notes with a descending scale
        // TODO: if intervals are provided, the first interval is enough to learn the polarity
    }
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
            let our_delta = lhs.semitone_delta(*rhs);
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
    #[test]
    fn c_base_test() {
        let mut c_base_test = Scale {
            names: vec!["Indefinite Note Scale".to_string()],
            notes: None,
            intervals: Some(vec![2, 2, 1, 2, 2, 2, 1]),
        };
        let mut d_base_test = Scale {
            names: vec!["D(efinite) Note Scale".to_string()],
            notes: Some(vec![note!(D)]),
            intervals: Some(vec![2, 2, 1, 2, 2, 2, 1]),
        };
        c_base_test.fill_blanks();
        d_base_test.fill_blanks();
        assert_eq!(
            c_base_test.notes.unwrap()[0],
            note!(C),
            "Did not assume scale with no notes started on C"
        );
        assert_eq!(
            d_base_test.notes.unwrap()[0],
            note!(D),
            "Scale with first note defined as D somehow wound up starting somewhere else"
        );
    }
}
