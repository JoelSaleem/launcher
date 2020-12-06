use crate::app::App;
use std::fs;
use tui::style::Color;

#[derive(Debug, Clone)]
pub struct Repo {
    pub id: String,
    pub path: String,
    pub colour: Color,
    pub keyword: String,
}

impl Repo {
    pub fn new() -> Repo {
        Repo {
            id: String::new(),
            path: String::new(),
            colour: Color::White,
            keyword: String::new(),
        }
    }

    pub fn read_from_settings(app: &mut App) {
        // Read repos from settings.yaml
        let f = fs::read_to_string("/Users/joelsaleem/.gitlauncher/settings.yaml")
            .expect("could not read settings.yaml");
        let settings = yaml_rust::YamlLoader::load_from_str(&f).unwrap();
        let repo_data = settings[0]["repos"].as_vec().unwrap();

        // Iterate through data and create Repo objs in App.repos
        for r in repo_data {
            let data = r.as_hash().unwrap();
            for (_, v) in data {
                let v = v.as_hash().unwrap();
                let mut repo = Repo::new();

                for (key, val) in v.iter() {
                    match key.as_str().unwrap() {
                        "id" => repo.id = String::from(val.as_str().unwrap()),
                        "path" => repo.path = String::from(val.as_str().unwrap()),
                        "colour" => {
                            let col_data = val.as_hash().unwrap();
                            let mut red: u8 = 0;
                            let mut green: u8 = 0;
                            let mut blue: u8 = 0;
                            for (col, val) in col_data.iter() {
                                match col.as_str().unwrap() {
                                    "r" => {
                                        red = val.as_i64().unwrap() as u8;
                                    }
                                    "g" => {
                                        green = val.as_i64().unwrap() as u8;
                                    }
                                    "b" => {
                                        blue = val.as_i64().unwrap() as u8;
                                    }
                                    _ => {}
                                }
                            }
                            repo.colour = Color::Rgb(red, green, blue);
                        }
                        "keyword" => repo.keyword = String::from(val.as_str().unwrap()),
                        _ => {}
                    }
                }

                app.repos.push(repo)
            }
        }
    }
}
