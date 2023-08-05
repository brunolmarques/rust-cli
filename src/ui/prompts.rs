use crate::{models::Project, io_utils::get_user_input, templates::ProgrammingLanguage};

pub struct Prompts {
    pub create_project: Box<dyn Fn(ProgrammingLanguage) -> Project>,
    pub create_edit_project: Box<dyn Fn(Project) -> Project>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_project: Box::new(create_project_prompt),
            create_edit_project: Box::new(create_edit_project_prompt),
        }
    }
}

fn create_project_prompt(language: ProgrammingLanguage) -> Project {
    println!("---------------------- NEW PROJECT TEMPLATE ----------------------");
    println!("\nPROJECT NAME: ");

    let project_name = get_user_input();

    println!("\nPROJECT DESCRIPTION: ");

    let project_desc = get_user_input();

    println!("\nPROJECT OWNER: ");

    let project_owner = get_user_input();

    println!("\nPROJECT OWNER EMAIL: ");

    let project_owner_email = get_user_input();

    let project = Project { 
        language,
        name: project_name,
        description: project_desc,
        owner: project_owner,
        owner_email: project_owner_email,
    };

    project
}


fn create_edit_project_prompt(project: Project) -> Project {
    println!("------------------------ EDIT PROJECT TEMPLATE -------------------------");
    println!();
    println!("Project Programming Language: {}", project.language);
    
    println!("\nOLD PROJECT NAME: {}", project.name);
    println!("NEW PROJECT NAME: ");
    let new_project_name = get_user_input();
    
    println!("\nOLD PROJECT DESCRIPTION: {}", project.description);
    println!("NEW PROJECT DESCRIPTION: ");
    let new_project_desc = get_user_input();
    
    println!("\nOLD PROJECT OWNER: {}", project.owner);
    println!("NEW PROJECT OWNER: ");
    let new_project_owner = get_user_input();
    
    println!("\nOLD PROJECT OWNER EMAIL: {}", project.owner_email);
    println!("NEW PROJECT OWNER EMAIL: ");
    let new_project_owner_email = get_user_input();

    let new_project = Project { 
        language: project.language,
        name: new_project_name,
        description: new_project_desc,
        owner: new_project_owner,
        owner_email: new_project_owner_email,
    };

    new_project
}