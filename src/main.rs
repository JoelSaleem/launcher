mod app;
mod config;
mod events;

use crate::events::{Event, Events};
use io::Result;
use std::fs;
use std::io;
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
    style::Style,
    text::{Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use termion::input::TermRead;
use yaml_rust;

#[derive(Debug)]
struct Repo {
    pub name: String,
    pub path: String,
    pub colour: String,
    pub keyword: String,
}

impl Repo {
    pub fn new() -> Repo {
        Repo {
            name: String::new(),
            path: String::new(),
            colour: String::new(),
            keyword: String::new(),
        }
    }
}

fn main() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut events = Events::new();
    let mut app = app::App::default();

    let f = fs::read_to_string("settings.yaml")?;
    let settings = yaml_rust::YamlLoader::load_from_str(&f).unwrap();
    let repo_data = settings[0]["repos"].as_vec().unwrap();

    let mut repos: Vec<Repo> = Vec::new();
    for a in repo_data {
        let data = a.as_hash().unwrap();
        for (k, v) in data {
            let v = v.as_hash().unwrap();
            let mut repo = Repo::new();
            for (key, val) in v.iter() {
                match key.as_str().unwrap() {
                    "name" => repo.name = String::from(val.as_str().unwrap()),
                    "path" => repo.path = String::from(val.as_str().unwrap()),
                    "colour" => repo.colour = String::from(val.as_str().unwrap()),
                    "keyword" => repo.keyword = String::from(val.as_str().unwrap()),
                    _ => {}
                }
            }

            repos.push(repo)
        }
    }

    println!("{:?}", repos);

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

            /*
             * TODO: PARSING
             */

            // let messages: Vec<ListItem> = app
            //     .messages
            //     .iter()
            //     .enumerate()
            //     .map(|(i, m)| {
            //         let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            //         ListItem::new(content)
            //     })
            //     .collect();
            // let messages =
            //     List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
            // f.render_widget(messages, chunks[2]);
        })?;

        // Handle input
        if let Event::Input(input) = events.next().expect("none") {
            match input {
                Key::Esc => {
                    break;
                    // events.enable_exit_key();
                }
                Key::Char('\n') => {
                    println!("rtn");
                    // app.messages.push(app.input.drain(..).collect());
                }
                Key::Char(c) => {
                    println!("char {}", c);
                    // app.input.push(c);
                }
                Key::Backspace => {
                    println!("backspace");
                    // app.input.pop();
                }
                _ => {}
            }
        }
    }
    Ok(())
}
