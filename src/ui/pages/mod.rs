use crate::models::{Action, Project};
use crate::templates::gen_map;
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::{any::Any, fs};

mod page_helpers;
use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        let ascii_art = fs::read_to_string("src/ui/pages/ascii_art.txt").unwrap();
        let ascii_description = fs::read_to_string("src/ui/pages/ascii_description.txt").unwrap();

        println!("{}", ascii_art.cyan());
        println!("{}", ascii_description.white());

        println!();
        println!();

        println!("-----------------------------------------------------------------------------------");
        println!("| ID |        ACTION         |                     DESCRIPTION                     |");
        println!("-----------------------------------------------------------------------------------");
        println!("| 1  | New project template  | Create a standard project template from repo.       |");
        println!("| 2  | Resource deployment   | Create resource blueprint used for deployment (IaC).|");
        println!("-----------------------------------------------------------------------------------");

        println!();
        println!();

        println!("[q] quit | [:id:] choose action");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "q" | "Q" => Ok(Some(Action::Exit)),
            "c" | "C" => Ok(Some(Action::CancelAction)),
            input => {
                if let Ok(action_id) = input.parse::<u32>() {
                    match action_id {
                        1 => return Ok(Some(Action::NavigateToProjectTemplate)),
                        2 => return Ok(Some(Action::NavigateToResourceDeployment)),
                        _ => return Ok(None),
                    }
                }
                Ok(None)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct TemplateSelection {}

impl Page for TemplateSelection {
    fn draw_page(&self) -> Result<()> {
        clearscreen::clear().unwrap();

        println!("---------------------------- TEMPLATES ---------------------------");
        println!(" DESCRIPTION: Create a new project directory using the standard   ");
        println!(" template stored at the central repository.                       ");
        
        println!();
        println!();

        println!("-----------------------------------");
        println!("|  ID  |   PROGRAMMING LANGUAGE   |");
        println!("-----------------------------------");

        gen_map().iter().for_each(|(id, prog_lang)|{
            println!(
                "|  {i}|        {p}|",
                i = get_column_string(&id.to_string(), 4),
                p = get_column_string(&prog_lang.to_string(), 18)
            )
        });
        println!("-----------------------------------");

        println!();
        println!();

        println!("[p] previous | [c] cancel operation | [:id:] select programming language");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let lang_map = gen_map();
        
        match input {
            "p" | "P" => Ok(Some(Action::NavigateToPreviousPage)),
            "c" | "C" => Ok(Some(Action::CancelAction)),
            input => {
                if let Ok(language_id) = input.parse::<u32>() {
                    if lang_map.contains_key(&language_id) {
                        return Ok(Some(Action::PickProgrammingLang {
                            language: *lang_map.get(&language_id).unwrap(),
                        }));
                    }
                }
                Ok(None)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ProjectTemplate {
    pub project: Project
}

impl Page for ProjectTemplate {
    fn draw_page(&self) -> Result<()> {
        clearscreen::clear().unwrap();

        println!("---------------------- NEW PROJECT TEMPLATE ----------------------");
        println!();
        println!("Project Programming Language: {}", self.project.language);
        println!("Project Name: {}", self.project.name);
        println!("Project Description: {}", self.project.description);
        println!("Project Owner: {}", self.project.owner);
        
        println!();
        println!();

        println!("Proceed with template creation? [y/n] ");

        Ok(())

    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {       
        match input {
            "y" | "Y" => Ok(Some(Action::NavigateToPreviousPage)),
            "n" | "N" => Ok(Some(Action::NavigateToPreviousPage)),
            input => Ok(None)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}