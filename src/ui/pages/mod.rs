use crate::models::Action;
use crate::templates::gen_map;
use anyhow::anyhow;
use anyhow::Result;
use colored::Colorize;
use std::any::Any;
use std::fs;

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

        println!("---------------------------------------------------------------------------------");
        println!("| ID |        ACTION       |                     DESCRIPTION                     |");
        println!("---------------------------------------------------------------------------------");
        println!("| 1  | Project template    | Create a standard project template from repo.       |");
        println!("| 2  | Resource deployment | Create resource blueprint used for deployment (IaC).|");
        println!("---------------------------------------------------------------------------------");

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
        println!(" DESCRIPTION: Create a new project folder using the standard      ");
        println!(" template stored on the central repository.                       ");
        
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
        match input {
            "p" | "P" => Ok(Some(Action::NavigateToPreviousPage)),
            "c" | "C" => Ok(Some(Action::CancelAction)),
            input => {
                if let Ok(language_id) = input.parse::<u32>() {
                    if gen_map().contains_key(&language_id) {
                        return Ok(Some(Action::PickProgrammingLang {
                            language_id: language_id,
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
