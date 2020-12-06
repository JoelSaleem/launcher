mod app;
mod config;
mod events;
mod repo;
mod terminal;

use crate::events::{Event, Events};
use crate::repo::Repo;
use crate::terminal::build_terminal;

use io::Result;
use std::fs;
use std::io;
use std::process::Command;
use termion::event::Key;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use termion::input::TermRead; // Do not delete
use yaml_rust;

fn main() -> Result<()> {
    let mut terminal = build_terminal().unwrap();

    let events = Events::new();
    let mut app = app::App::default();
    Repo::read_from_settings(&mut app);

    if app.repos.len() == 0 {
        panic!("No repos set in the settings.yaml file");
    }

    loop {
        app.update_filtered_repos();

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

            let mut text = Text::from(Spans::from(
                "Launcher (press `esc` to quit, `enter` to open code, type to `search`)",
            ));
            text.patch_style(Style::default());
            let help_message = Paragraph::new(text);
            f.render_widget(help_message, chunks[0]);

            let input = Paragraph::new(app.search_str.as_ref())
                .style(Style::default())
                .block(Block::default().borders(Borders::ALL).title("Search"));
            f.render_widget(input, chunks[1]);

            let list_items: Vec<ListItem> = app
                .filtered_repos
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                    let should_highlight = app.selected_idx == idx;
                    let style = if should_highlight {
                        Style::default().bg(item.colour)
                    } else {
                        Style::default().fg(item.colour)
                    };

                    ListItem::new(Spans::from(Span::styled(&item.id, style)))
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
                    if app.selected_idx < app.filtered_repos.len() {
                        let selected_repo = &app.filtered_repos[app.selected_idx];
                        Command::new("sh")
                            .arg("-c")
                            .arg(format!("{} {}", selected_repo.keyword, selected_repo.path))
                            .output()
                            .unwrap();
                        break;
                    }
                }
                Key::Down => {
                    if app.filtered_repos.len() > 0 {
                        app.selected_idx = (app.selected_idx + 1) % app.filtered_repos.len();
                    }
                }
                Key::Up => {
                    if app.filtered_repos.len() > 0 {
                        app.selected_idx = (app.selected_idx + app.filtered_repos.len() - 1)
                            % app.filtered_repos.len();
                    }
                }
                Key::Char(c) => {
                    app.search_str.push(c);
                }
                Key::Backspace => {
                    app.search_str.pop();
                }
                _ => {}
            }
        }
    }
    Ok(())
}
