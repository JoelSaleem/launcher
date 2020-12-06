use crate::app::App;
use crate::config::Config;
use std::io;
use std::process::Command;
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub enum Action {
    Break,
    Continue,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>, // WIP -- SWITCHING BETWEEN SEARCH AND LIST
    ignore_exit_key: Arc<AtomicBool>,
    tick_handle: thread::JoinHandle<()>,
}
impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let tx = tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                        if !ignore_exit_key.load(Ordering::Relaxed) && key == config.exit_key {
                            return;
                        }
                    }
                }
            })
        };
        let tick_handle = {
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };
        Events {
            rx,
            ignore_exit_key,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }

    fn handle_enter_press(&self, app: &App) -> Action {
        if app.selected_idx < app.filtered_repos.len() as i8 {
            let selected_repo = &app.filtered_repos[app.selected_idx as usize];
            Command::new("sh")
                .arg("-c")
                .arg(format!("{} {}", selected_repo.keyword, selected_repo.path))
                .output()
                .unwrap();
            return Action::Break;
        }
        return Action::Continue;
    }

    fn handle_list_scroll(&self, app: &mut App, scroll_dir: i8) -> Action {
        if app.filtered_repos.len() > 0 {
            app.selected_idx = (app.selected_idx + (app.filtered_repos.len() as i8) + scroll_dir)
                % (app.filtered_repos.len() as i8);
        }
        Action::Continue
    }

    pub fn handle_user_input(&self, app: &mut App) -> Action {
        if let Event::Input(input) = self.next().expect("none") {
            match input {
                Key::Esc => Action::Break,
                Key::Char('\n') => self.handle_enter_press(app),
                Key::Down => self.handle_list_scroll(app, 1),
                Key::Up => self.handle_list_scroll(app, -1),
                Key::Char(c) => {
                    app.search_str.push(c);
                    Action::Continue
                }
                Key::Backspace => {
                    app.search_str.pop();
                    Action::Continue
                }
                _ => Action::Continue,
            }
        } else {
            Action::Continue
        }
    }
}
