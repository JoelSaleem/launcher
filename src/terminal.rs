use std::io;
use std::io::Stdout;
use termion::{input::MouseTerminal, raw::IntoRawMode, raw::RawTerminal, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

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
