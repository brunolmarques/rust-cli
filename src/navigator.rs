use crate::{
    models::Action,
    ui::{pages::*, Prompts},
};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    prompts: Prompts,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            pages: vec![Box::new(HomePage {})],
            prompts: Prompts::new(),
        }
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        self.pages.last()
    }

    // Navigation actions behaviors are defined here 
    pub fn handle_action(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::NavigateToProjectTemplate => {
                let page_view = TemplateSelection {};
                self.pages.push(Box::new(page_view));
            },
            Action::NavigateToPreviousPage => {
                if !self.pages.is_empty() {
                    self.pages.pop();
                }
            },
            Action::CancelAction => {
                if !self.pages.is_empty() {
                    self.pages.clear();
                    clearscreen::clear().unwrap();
                    self.pages.push(Box::new(HomePage {}));
                }
            },
            Action::PickProgrammingLang { language } => {
                let project = (self.prompts.create_project)(language);
                let page_view = ProjectTemplate { project };
                self.pages.push(Box::new(page_view));
            },
            Action::EditProjectData { project } => {
                let project = (self.prompts.create_edit_project)(project);
                let page_view = ProjectTemplate { project };
                self.pages.push(Box::new(page_view));
            },
            Action::ProjectCreationPage { project } => {
                todo!();
                // let page_view = ProjectTemplate { project };
                // self.pages.push(Box::new(page_view));
            },
            Action::Exit => {
                self.pages.clear();
            },
            _ => {return Err("General error!".to_owned())}
        }
        Ok(())
    }
}