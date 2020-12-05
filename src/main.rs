mod app;
mod config;
mod events;
mod repo;

use crate::repo::Repo;

use crate::events::{Event, Events};
use io::Result;
use std::fs;
use std::io;
use std::process::Command;
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use termion::input::TermRead;
use yaml_rust;

fn main() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut events = Events::new();
    let mut app = app::App::default();

    let f = fs::read_to_string("settings.yaml").expect("could not read settings.yaml");
    let settings = yaml_rust::YamlLoader::load_from_str(&f).unwrap();
    let repo_data = settings[0]["repos"].as_vec().unwrap();

    for a in repo_data {
        let data = a.as_hash().unwrap();
        for (k, v) in data {
            let v = v.as_hash().unwrap();
            let mut repo = Repo::new();
            for (key, val) in v.iter() {
                match key.as_str().unwrap() {
                    "name" => repo.name = String::from(val.as_str().unwrap()),
                    "path" => repo.path = String::from(val.as_str().unwrap()),
                    "colour" => {
                        let col_data = val.as_hash().unwrap();
                        let mut red: u8 = 0;
                        let mut green: u8 = 0;
                        let mut blue: u8 = 0;
                        for (col, val) in col_data.iter() {
                            match col.as_str().unwrap() {
                                "r" => {
                                    red = val.as_i64().unwrap() as u8;
                                }
                                "g" => {
                                    green = val.as_i64().unwrap() as u8;
                                }
                                "b" => {
                                    blue = val.as_i64().unwrap() as u8;
                                }
                                _ => {}
                            }
                        }
                        repo.colour = Color::Rgb(red, green, blue);
                    }
                    "keyword" => repo.keyword = String::from(val.as_str().unwrap()),
                    _ => {}
                }
            }

            app.repos.push(repo)
        }
    }

    // println!("{:?}", repos);
    if app.repos.len() == 0 {
        panic!("No repos set in the settings.yaml file");
    }

    loop {
        // Draw UI
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let mut text = Text::from(Spans::from("Launcher"));
            text.patch_style(Style::default());
            let help_message = Paragraph::new(text);
            f.render_widget(help_message, chunks[0]);

            let input = Paragraph::new(app.search_str.as_ref())
                .style(Style::default())
                .block(Block::default().borders(Borders::ALL).title("Search"));
            f.render_widget(input, chunks[1]);

            let list_items: Vec<ListItem> = app
                .repos
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                    let should_highlight = app.selected_idx == idx;
                    let style = if should_highlight {
                        Style::default().bg(item.colour)
                    } else {
                        Style::default().fg(item.colour)
                    };

                    ListItem::new(Spans::from(Span::styled(&item.name, style)))
                })
                .collect();
            let repo_list =
                List::new(list_items).block(Block::default().borders(Borders::ALL).title("Repos"));
            f.render_widget(repo_list, chunks[2]);
        })?;

        // Handle input
        if let Event::Input(input) = events.next().expect("none") {
            match input {
                Key::Esc => {
                    break;
                }
                Key::Char('\n') => {
                    let selected_repo = &app.repos[app.selected_idx];
                    Command::new("sh")
                        .arg("-c")
                        .arg(format!("{} {}", selected_repo.keyword, selected_repo.path))
                        .output()
                        .unwrap();
                    break;
                }
                Key::Down => {
                    app.selected_idx = (app.selected_idx + 1) % app.repos.len();
                }
                Key::Up => {
                    app.selected_idx = (app.selected_idx + app.repos.len() - 1) % app.repos.len();
                }
                // Key::Char(c) => {
                //     println!("char {}", c);
                // }
                // Key::Backspace => {
                //     println!("backspace");
                // }
                _ => {}
            }
        }
    }
    Ok(())
}
