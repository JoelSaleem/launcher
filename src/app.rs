use crate::repo::Repo;
use std::default::Default;

pub struct App {
    pub search_str: String,
    pub selected_idx: usize,
    pub repos: Vec<Repo>,
    pub filtered_repos: Vec<Repo>,
}

impl Default for App {
    fn default() -> App {
        App {
            search_str: String::new(),
            selected_idx: 0,
            repos: Vec::new(),
            filtered_repos: Vec::new(),
        }
    }
}
