//! Big Bag Of Words
//!
//! The "Big Bag Of Words" is used in text analysis and
//! machine learning.  It reduces a text to a collection of
//! words, each with a count of the number of occurrences.
//!
//! This implementation uses zero-copy strings when
//! reasonably possible to improve performance and reduce
//! memory usage.
//!
//! Words are separated by whitespace, and consist of a
//! span of one or more consecutive letters (any Unicode
//! code point in the "letter" class) with no internal
//! punctuation: leading and trailing punctuation are
//! removed.
//!
//! For example, the text
//!
//! ```text
//! "It ain't over untïl it ain't, over."
//! ```
//!
//! contains the sequence of words `"It"`, `"over"`,
//! `"untïl"`, `"it"`, `"over"`.
//!
//! Words in the bag containing uppercase letters will be
//! represented by their lowercase equivalent.

use std::borrow::Cow;
use std::collections::BTreeMap;

/// Each key in this struct's map is a word in some
/// in-memory text document. The corresponding value is the
/// count of occurrences.
#[derive(Debug, Default, Clone)]
pub struct Bbow<'a>(BTreeMap<Cow<'a, str>, usize>);

fn is_word(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_alphabetic())
}

#[test]
fn test_is_word() {
    assert_eq!(is_word("word"), true);
    assert_eq!(is_word("Bigword"), true);
    assert_eq!(is_word("REALLYBIGWORD"), true);
    assert_eq!(is_word("withUnicodƹ"), true);
    assert_eq!(is_word("not-a-word"), false);
    assert_eq!(is_word("n0taword"), false);
    assert_eq!(is_word("notaword!"), false);
    assert_eq!(is_word(""), false);
}

fn has_uppercase(word: &str) -> bool {
    word.chars().any(char::is_uppercase)
}

#[test]
fn test_has_uppercase() {
    assert_eq!(has_uppercase("Bigword"), true);
    assert_eq!(has_uppercase("REALLYBIGWORD"), true);
    assert_eq!(has_uppercase("withUnicodƹ"), true);
    assert_eq!(has_uppercase("word"), false);
    assert_eq!(has_uppercase("w"), false);
    assert_eq!(has_uppercase("!"), false);
    assert_eq!(has_uppercase(""), false);
}

fn trim_punctuation(word: &str) -> &str {
    // Trim any characters that are not alphabetic
    word.trim_matches(|c: char| !c.is_alphabetic())
}

#[test]
fn test_trim_punctuation() {
    assert_eq!(trim_punctuation("word!"), "word");
    assert_eq!(trim_punctuation("word?"), "word");
    assert_eq!(trim_punctuation(".word"), "word");
    assert_eq!(trim_punctuation("word."), "word");
    assert_eq!(trim_punctuation("¡word"), "word");
    assert_eq!(trim_punctuation("unicodƐ"), "unicodƐ");
}

impl<'a> Bbow<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this BBOW.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up a BBOW covering
    /// multiple texts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new().extend_from_text("Hello world.");
    /// assert_eq!(2, bbow.len());
    /// assert_eq!(1, bbow.match_count("hello"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        target
            .split_whitespace()
            // trim_punctuation gets new string slices into target
            // that trim (remove leading or trailing) ascii punctuation
            .map(trim_punctuation)
            // filter removes any words that fail the is_word boolean check
            .filter(|w| is_word(w))
            // Return a new, owned lowercase string if an uppercase is present,
            // otherwise return a borrowed version
            .for_each(|w| {
                let key = if has_uppercase(w) {
                    Cow::from(w.to_lowercase())
                } else {
                    Cow::from(w)
                };

                *self.0.entry(key).or_insert(0) += 1;
            });

        self
    }

    /// Report the number of occurrences of the given
    /// `keyword` that are indexed by this BBOW. The keyword
    /// should be lowercase and not contain punctuation, as
    /// per the rules of BBOW: otherwise the keyword will
    /// not match and 0 will be returned.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("b b b-banana b");
    /// assert_eq!(3, bbow.match_count("b"));
    /// ```
    pub fn match_count(&self, keyword: &str) -> usize {
        match self.0.get(keyword) {
            Some(&num) => num,
            None => 0,
        }
    }

    pub fn words(&'a self) -> impl Iterator<Item = &'a str> {
        self.0.keys().map(|w| w.as_ref())
    }

    /// Count the overall number of words contained in this BBOW:
    /// multiple occurrences are considered separate.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(3, bbow.count());
    /// ```
    pub fn count(&self) -> usize {
        // Iterate over all the entries in the BTreeMap
        // and sum the entry values
        self.0.values().sum()
    }

    /// Count the number of unique words contained in this BBOW,
    /// not considering number of occurrences.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(2, bbow.len());
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is this BBOW empty?
    ///
    /// # Examples
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new().extend_from_text("Hello world.");
    /// assert_eq!(false, bbow.is_empty());
    /// let bbow = Bbow::new();
    /// assert_eq!(true, bbow.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
