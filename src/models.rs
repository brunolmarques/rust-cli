#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToProjectTemplate,
    PickProgrammingLang { language_id: u32 },
    NavigateToPreviousPage,
    NavigateToResourceDeployment,
    CancelAction,
    Exit,
}
