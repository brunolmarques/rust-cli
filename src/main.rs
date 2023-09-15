mod ui;
mod models;
mod navigator;
mod templates;
mod project_starter;

use navigator::*;

mod io_utils;
use io_utils::*;


fn main() {
    let mut navigator = Navigator::new();

    loop {
        clearscreen::clear().unwrap();

        // get current page from navigator. If there is no current page exit the loop.
        while let Some(page) = navigator.get_current_page() {
            // render page
            if let Err(error) = page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            }

            // get user input
            let user_input = get_user_input();

            // pass input to page's input handler
            // if the page's input handler returns an action let the navigator process the action
            match page.handle_input(user_input.trim()) {
                Err(e) => {
                    println!(
                        "Error getting user input: {}\nPress any key to continue...",
                        e
                    );
                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(e) = navigator.handle_action(action) {
                            println!("Error handling and processiing user input: {}\nPress any key to continue...", e);
                            wait_for_key_press();
                        }
                    }
                }
            }
        }
        clearscreen::clear().unwrap();
        return
    }
}