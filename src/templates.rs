use std::{collections::BTreeMap, fmt::Display};

// Definition of programming langagues templates
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ProgrammingLanguage {
    Rust,
    Scala,
    Python,
    Java
}

impl Display for ProgrammingLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Rust => write!(f, "Rust"),
            Self::Scala => write!(f, "Scala"),
            Self::Python => write!(f, "Python"),
            Self::Java => write!(f, "Java"),
        }
    }
}


pub fn gen_map() -> BTreeMap<u32, ProgrammingLanguage> {
    let mut lang_map = BTreeMap::new();
    lang_map.insert(1, ProgrammingLanguage::Rust);
    lang_map.insert(2, ProgrammingLanguage::Scala);
    lang_map.insert(3, ProgrammingLanguage::Python);
    lang_map.insert(4, ProgrammingLanguage::Java);
    lang_map
}
