use ::phf::{Map, phf_map};

pub enum ProgrammingLanguages {
    Scala,
    Python,
    Rust,
    Java
}


pub const TEMPLATES_MAP: Map<&i32, ProgrammingLanguages> = phf_map! {
    1 => Scala,
    2 => Python,
    3 => Rust,
    4 => Java,
};