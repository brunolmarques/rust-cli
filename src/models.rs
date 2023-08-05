use crate::templates::ProgrammingLanguage;

// Definition of all possible CLI actions
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToProjectTemplate,
    PickProgrammingLang { language: ProgrammingLanguage },
    EditProjectData { project: Project},
    NavigateToPreviousPage,
    NavigateToProjectCreation,
    NavigateToResourceDeployment,
    CancelAction,
    Exit,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Project {
    pub language: ProgrammingLanguage,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub owner_email: String,
}

impl Project {
    pub fn new(language: ProgrammingLanguage) -> Self {
        Self {
            language: language,
            name: String::new(),
            description: String::new(),
            owner: String::new(),
            owner_email: String::new(),
        }
    }
}