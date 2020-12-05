use std::default::Default;

pub struct App {
    pub search_str: String,
    pub selected_id: String,
    pub items: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            search_str: String::new(),
            selected_id: String::new(),
            items: Vec::new(),
        }
    }
}
