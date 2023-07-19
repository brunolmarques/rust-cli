use crate::{models::Project, io_utils::get_user_input, templates::ProgrammingLanguages};

pub struct Prompts {
    pub create_project: Box<dyn Fn(ProgrammingLanguages) -> Project>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_project: Box::new(create_project_prompt)
        }
    }
}

fn create_project_prompt(language: ProgrammingLanguages) -> Project {
    println!("---------------------- NEW PROJECT TEMPLATE ----------------------");
    println!();
    println!("Project Name: ");

    let project_name = get_user_input();

    println!("Project Description: ");

    let project_desc = get_user_input();

    println!("Project Owner: ");

    let project_owner = get_user_input();

    let project = Project { 
        language,
        name: project_name,
        description: project_desc,
        owner: project_owner,
    };

    project
}