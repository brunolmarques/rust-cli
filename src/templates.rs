use std::{collections::BTreeMap, fmt::Display};


pub enum ProgrammingLanguages {
    Rust,
    Scala,
    Python,
    Java
}

impl Display for ProgrammingLanguages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Rust => write!(f, "Rust"),
            Self::Scala => write!(f, "Scala"),
            Self::Python => write!(f, "Python"),
            Self::Java => write!(f, "Java"),
        }
    }
}


pub fn gen_map() -> BTreeMap<u32, ProgrammingLanguages> {
    let mut lang_map = BTreeMap::new();
    lang_map.insert(1, ProgrammingLanguages::Rust);
    lang_map.insert(2, ProgrammingLanguages::Scala);
    lang_map.insert(3, ProgrammingLanguages::Python);
    lang_map.insert(4, ProgrammingLanguages::Java);
    lang_map
}
