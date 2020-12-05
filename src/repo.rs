use tui::style::Color;

#[derive(Debug)]
pub struct Repo {
    pub name: String,
    pub path: String,
    pub colour: Color,
    pub keyword: String,
}

impl Repo {
    pub fn new() -> Repo {
        Repo {
            name: String::new(),
            path: String::new(),
            colour: Color::White,
            keyword: String::new(),
        }
    }
}
