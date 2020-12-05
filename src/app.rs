use std::default::Default;

pub enum Mode {
    Searching,
    Selecting,
}

pub struct App {
    pub search_str: String,
    pub selected_idx: usize,
    pub items: Vec<String>,
    pub mode: Mode,
}

impl Default for App {
    fn default() -> App {
        App {
            search_str: String::new(),
            selected_idx: 0,
            items: Vec::new(),
            mode: Mode::Selecting,
        }
    }
}
