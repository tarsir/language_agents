use crate::meaning::Meaning;

pub enum Category {
  Start(Meaning),
  MidCategory(Meaning),
}
