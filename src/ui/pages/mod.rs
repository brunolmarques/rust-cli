use std::any::Any;
use std::fs;
use anyhow::anyhow;
use anyhow::Result;
use colored::Colorize;
use crate::models::Action;
use crate::templates::TEMPLATES_MAP;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        let ascii_art = fs::read_to_string("ascii_art.txt").unwrap();
        println!("{}", ascii_art.cyan());

        println!();
        println!();

        println!("[q] quit | [c] cancel operation | [:id:] action to execute");

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
                        _ => return Ok(None)
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
        println!("---------------------------- TEMPLATES ---------------------------");
        println!("  id  |     programming language     |         description        ");

      
        println!();
        println!();

        println!("[p] previous | [c] cancel operation | [:id:] select programming language");

    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        match input {
            "p" | "P" => Ok(Some(Action::NavigateToPreviousPage)),
            "c" | "C" => Ok(Some(Action::CancelAction)),
            input => {
                if let Ok(language_id) = input.parse::<u32>() {
                    if TEMPLATES_MAP.contains_key(&language_id) {
                        return Ok(Some(Action::NavigateToProgrammingLang { 
                            language_id: language_id 
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