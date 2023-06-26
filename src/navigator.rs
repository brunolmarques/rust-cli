use crate::{
    models::Action,
    ui::pages::{HomePage, Page, TemplateSelection},
};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            pages: vec![Box::new(HomePage {})],
        }
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        self.pages.last()
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::NavigateToProjectTemplate => {
                let page_view = TemplateSelection {};
                self.pages.push(Box::new(page_view));
            },
            Action::Exit => {
                self.pages.clear();
            }
        }
        Ok(())
    }
}