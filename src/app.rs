use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use crate::{windows::score_window, color_scheme::ColorScheme};
use crate::bjj_match::BjjMatch;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppCommon {
    pub show_score_window: bool,
    //All application information is stored here.
    pub color_scheme: ColorScheme,
    pub matches: Vec<BjjMatch>
}

pub enum WindowEvent {
    NewScorePanel,
    None
}

impl AppCommon {
    pub fn from_project_dirs() -> Self {
        if let Some(proj_dirs) = ProjectDirs::from("au", "popplestones", env!("CARGO_PKG_NAME")) {
            return Self {
                show_score_window: false,
                color_scheme: AppCommon::read_color_scheme(proj_dirs.config_dir().join("color_scheme.toml")),
                matches: AppCommon::load_matches(proj_dirs.config_dir().join("matches.toml"))
            };
        } else {
            eprintln!("Could not find project directory");
        }

        Self {
            show_score_window: false,
            color_scheme: Default::default(),
            matches: vec![]
        }
    }
    fn read_color_scheme(file: PathBuf) -> ColorScheme {
        match fs::read_to_string(&file) {
            Ok(contents) => {
                match toml::from_str::<ColorScheme>(&contents) {
                    Ok(color_scheme) => { return color_scheme; },
                    Err(e) => { eprintln!("Error parsing TOML: {:?}", e); }
                }
            },
            Err(e) => {
                eprintln!("Error reading file: {:?}\n{:?}", file, e);
            }
        }

        ColorScheme::default()
    }

    fn load_matches(file: PathBuf) -> Vec<BjjMatch>
    {
        match fs::read_to_string(&file) {
            Ok(contents) => {
                match toml::from_str::<Vec<BjjMatch>>(&contents) {
                    Ok(matches) => { return matches; },
                    Err(e) => { eprintln!("Error parsing TOML: {:?}", e); }
                }
            },
            Err(e) => { eprintln!("Error reading file: {:?}\n{:?}", file, e); }
        }

        vec![]
    }
}

impl egui_multiwin::multi_window::CommonEventHandler<AppCommon, WindowEvent> for AppCommon {
    fn process_event(&mut self, event: WindowEvent) -> Vec<egui_multiwin::multi_window::NewWindowRequest<AppCommon>> {
        let mut windows_to_create = vec![];

        match event {
            WindowEvent::NewScorePanel => windows_to_create.push(score_window::ScoreWindow::request("Score Window".to_string())),
            WindowEvent::None => {}
        }
        windows_to_create
    }
}
