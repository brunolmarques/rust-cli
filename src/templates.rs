use std::collections::HashMap;


pub enum ProgrammingLanguages {
    Rust,
    Scala,
    Python,
    Java
}


pub fn gen_map() -> HashMap<u32, ProgrammingLanguages> {
    let mut lang_map = HashMap::new();
    lang_map.insert(1, ProgrammingLanguages::Rust);
    lang_map.insert(1, ProgrammingLanguages::Scala);
    lang_map.insert(1, ProgrammingLanguages::Python);
    lang_map.insert(1, ProgrammingLanguages::Java);
    lang_map
}
