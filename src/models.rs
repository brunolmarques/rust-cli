#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToProjectTemplate,
    NavigateToProgrammingLang { language_id: u32 },
    NavigateToPreviousPage,
    NavigateToResourceDeployment,
    CancelAction,
    Exit,
}
