use std::io;
use std::io::Stdout;
use termion::{input::MouseTerminal, raw::IntoRawMode, raw::RawTerminal, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn build_terminal() -> Result<
    Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>,
    std::io::Error,
> {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);

    let backend = TermionBackend::new(stdout);
    return Terminal::new(backend);
}

pub fn draw_terminal_ui(
    terminal: &mut Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>,
    app: &App,
) {
    terminal
        .draw(|f| {
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
        })
        .unwrap();
}
