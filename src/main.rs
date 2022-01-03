mod app;
mod config;
mod events;
mod repo;
mod terminal_utils;

use crate::events::Events;
use crate::repo::Repo;
use crate::terminal_utils::build_terminal;
use events::Action;

use io::Result;
use std::env;
use std::io;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut terminal = build_terminal().unwrap();

    let events = Events::new();
    let mut app = app::App::new(String::from(path));
    Repo::read_from_settings(&mut app);

    if app.repos.len() == 0 {
        panic!("No repos set in the settings.yaml file");
    }

    loop {
        app.update_filtered_repos();

        terminal_utils::draw_terminal_ui(&mut terminal, &app);

        match events.handle_user_input(&mut app) {
            Action::Break => break,
            Action::Continue => {}
        }
    }
    Ok(())
}
