use crate::templates::ProgrammingLanguages;

// Definition of all possible CLI actions
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToProjectTemplate,
    PickProgrammingLang { language: ProgrammingLanguages },
    NavigateToPreviousPage,
    NavigateToResourceDeployment,
    CancelAction,
    Exit,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Project {
    pub language: ProgrammingLanguages,
    pub name: String,
    pub description: String,
    pub owner: String,
}

impl Project {
    pub fn new(language: ProgrammingLanguages) -> Self {
        Self {
            language: language,
            name: String::new(),
            description: String::new(),
            owner: String::new(),
        }
    }
}