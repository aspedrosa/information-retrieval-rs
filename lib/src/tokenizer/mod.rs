mod linguistic_rules;

use crate::tokenizer::linguistic_rules::LinguisticRule;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WHITE_SPACES: Regex = Regex::new(r"\s+").unwrap();
    static ref LESS_THAN_THREE: Regex = Regex::new(r"\b[[:alpha:]]{1,2}\b").unwrap();
    static ref PUNCT_EXCEPT_SLASH: Regex = Regex::new(r"[[:punct:]&&[^-]]").unwrap();
    static ref SLASH: Regex = Regex::new(r"-").unwrap();
    static ref ALL_NUM: Regex = Regex::new(r"^[[:digit:]]+$").unwrap();
}

trait Tokenizer {
    fn tokenize_string(&self, to_tokenize: String) -> Vec<String>;
    fn add_linguistic_rule(&mut self, rule: Box<dyn LinguisticRule>);
}

struct SimpleTokenizer {
    rules: Vec<Box<dyn LinguisticRule>>,
}

impl SimpleTokenizer {
    fn new() -> Self {
        SimpleTokenizer { rules: Vec::new() }
    }

    fn new_with_rules(rules: Vec<Box<dyn LinguisticRule>>) -> Self {
        SimpleTokenizer { rules }
    }
}

impl Tokenizer for SimpleTokenizer {
    fn tokenize_string(&self, to_tokenize: String) -> Vec<String> {
        let to_tokenize = to_tokenize
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphabetic() { c } else { ' ' })
            .collect::<String>();

        let to_split = LESS_THAN_THREE.replace_all(to_tokenize.as_str(), "");
        let to_split = to_split.trim();

        if to_split.is_empty() {
            return Vec::new();
        }

        let mut split = WHITE_SPACES
            .split(to_split)
            .map(String::from)
            .collect::<Vec<String>>();

        for rule in &self.rules {
            split = rule.apply(split);
        }

        split
    }

    fn add_linguistic_rule(&mut self, rule: Box<dyn LinguisticRule>) {
        self.rules.push(rule);
    }
}

struct AdvancedTokenizer {
    rules: Vec<Box<dyn LinguisticRule>>,
}

impl AdvancedTokenizer {
    fn new() -> Self {
        AdvancedTokenizer { rules: Vec::new() }
    }

    fn new_with_rules(rules: Vec<Box<dyn LinguisticRule>>) -> Self {
        AdvancedTokenizer { rules }
    }
}

impl Tokenizer for AdvancedTokenizer {
    fn tokenize_string(&self, to_tokenize: String) -> Vec<String> {
        let lower_case = to_tokenize.to_lowercase();
        let without_punct = PUNCT_EXCEPT_SLASH.replace_all(lower_case.as_str(), " ");
        let merged_slash = SLASH.replace_all(without_punct.as_ref(), "");
        let to_split = merged_slash.trim();

        let mut split = WHITE_SPACES
            .split(to_split)
            .map(String::from)
            .collect::<Vec<String>>();
        for rule in &self.rules {
            split = rule.apply(split);
        }

        split.into_iter().filter(|t| ALL_NUM.is_match(t)).collect()
    }

    fn add_linguistic_rule(&mut self, rule: Box<dyn LinguisticRule>) {
        self.rules.push(rule);
    }
}
