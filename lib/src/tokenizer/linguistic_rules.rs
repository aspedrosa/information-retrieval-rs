use rust_stemmers::Stemmer;
use std::collections::HashSet;

pub trait LinguisticRule {
    fn apply(&self, to_apply: Vec<String>) -> Vec<String>;
}

struct MinLengthRule {
    min_length: u16,
}

impl LinguisticRule for MinLengthRule {
    fn apply(&self, to_apply: Vec<String>) -> Vec<String> {
        to_apply
            .into_iter()
            .filter(|t| t.len() >= self.min_length as usize)
            .collect()
    }
}

impl MinLengthRule {
    fn new(min_length: u16) -> Self {
        MinLengthRule { min_length }
    }
}

struct StopWordsRule {
    stop_words: HashSet<String>,
}

impl LinguisticRule for StopWordsRule {
    fn apply(&self, to_apply: Vec<String>) -> Vec<String> {
        to_apply
            .into_iter()
            .filter(|t| !self.stop_words.contains(t))
            .collect()
    }
}

impl StopWordsRule {
    fn new(stop_words: HashSet<String>) -> Self {
        StopWordsRule { stop_words }
    }
}

struct SnowballStemmerRule {
    stemmer: Stemmer,
}

impl LinguisticRule for SnowballStemmerRule {
    fn apply(&self, to_apply: Vec<String>) -> Vec<String> {
        to_apply
            .into_iter()
            .map(|t| self.stemmer.stem(t.as_str()).to_string())
            .collect()
    }
}

impl SnowballStemmerRule {
    fn new(stemmer: Stemmer) -> Self {
        SnowballStemmerRule { stemmer }
    }
}
