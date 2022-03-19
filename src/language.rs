use std::collections::hash_map;
use std::option::Option;

use crate::category::Category;
use crate::meaning::Meaning;
use crate::signal::Signal;

pub struct Rule(Meaning, Signal);

pub struct Language(hash_map::HashMap<Category, Rule>);

impl Language {
  pub fn parse_utterance(&self, utterance: &str) -> Option<Meaning> {
    None
  }
}
