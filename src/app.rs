use crate::repo::Repo;
use std::default::Default;

pub struct App {
    pub search_str: String,
    pub selected_idx: i8,
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

impl App {
    /// Use app.search_str to filter repos
    pub fn update_filtered_repos(&mut self) {
        self.filtered_repos = Vec::new();
        for repo in self.repos.iter() {
            if repo.id.contains(&self.search_str) {
                self.filtered_repos.push(repo.clone());
            }
        }
    }
}
