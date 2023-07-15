use crate::bjj_match::{BjjMatch, Matches};
use crate::score_grid::{GridConfig, ScoreGrid};
use crate::{color_scheme::ColorScheme, windows::score_window};
use directories::ProjectDirs;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

const QUALIFIER: &str = "au";
const ORGANIZATION: &str = "popplestones";
const COLOR_SCHEME_FILE: &str = "color_scheme.toml";
const MATCHES_FILE: &str = "matches.toml";
const GRID_CONFIG_FILE: &str = "grid_config.toml";

#[derive(Debug)]
pub struct AppCommon {
    pub show_score_window: bool,
    //All application information is stored here.
    pub color_scheme: ColorScheme,
    pub matches: Vec<BjjMatch>,
    pub selected_match: Option<BjjMatch>,
    pub grid_config: GridConfig,
    pub score_grids: Option<ScoreGrid>,
    pub is_dirty: bool,
}

pub enum WindowEvent {
    NewScorePanel,
    None,
}

impl AppCommon {
    fn get_config_dir() -> Option<&'static Path> {
        ProjectDirs::from(QUALIFIER, ORGANIZATION, env!("CARGO_PKG_NAME"))
            .map(|proj_dirs| Box::leak(Box::new(proj_dirs.config_dir().to_path_buf())).as_path())
    }

    pub fn from_project_dirs() -> Self {
        if let Some(config_dir) = AppCommon::get_config_dir() {
            return Self {
                show_score_window: false,
                color_scheme: AppCommon::read_color_scheme(config_dir.join(COLOR_SCHEME_FILE)),
                matches: AppCommon::load_matches(config_dir.join(MATCHES_FILE)),
                selected_match: None,
                grid_config: AppCommon::load_grid_config(config_dir.join(GRID_CONFIG_FILE)),
                score_grids: None,
                is_dirty: false,
            };
        } else {
            eprintln!("Could not find project directory");
        }

        Self {
            show_score_window: false,
            color_scheme: Default::default(),
            matches: vec![],
            selected_match: None,
            grid_config: Default::default(),
            score_grids: None,
            is_dirty: false,
        }
    }

    pub fn save(&self) {
        if let Some(config_dir) = AppCommon::get_config_dir() {
            self.save_item(config_dir.join(COLOR_SCHEME_FILE), &self.color_scheme);
            self.save_item(config_dir.join(GRID_CONFIG_FILE), &self.grid_config);
            self.save_item(config_dir.join(COLOR_SCHEME_FILE), &self.color_scheme);

            let matches = Matches {
                matches: self.matches.clone(),
            };

            self.save_item(config_dir.join(MATCHES_FILE), &matches);
        } else {
            eprintln!("Could not find project directory");
        }
    }

    fn load_grid_config(file: PathBuf) -> GridConfig {
        match fs::read_to_string(&file) {
            Ok(contents) => match toml::from_str::<GridConfig>(&contents) {
                Ok(grid_config) => {
                    return grid_config;
                }
                Err(e) => {
                    eprintln!("Error parsing TOML: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Error reading file: {:?}\n{:?}", file, e);
            }
        }

        GridConfig::default()
    }

    fn save_item<T>(&self, file: PathBuf, item: &T)
    where
        T: serde::Serialize + Debug,
    {
        match toml::to_string::<T>(item) {
            Ok(serialized_string) => {
                if let Err(e) = fs::write(&file, serialized_string) {
                    eprintln!("Error writing to file: {:?}\n{:?}", file, e);
                }
            }
            Err(e) => {
                eprintln!("Cannot serialize: {:?}", item);
                eprintln!("Error serializing to TOML: {:?}", e);
            }
        }
    }

    fn read_color_scheme(file: PathBuf) -> ColorScheme {
        match fs::read_to_string(&file) {
            Ok(contents) => match toml::from_str::<ColorScheme>(&contents) {
                Ok(color_scheme) => {
                    return color_scheme;
                }
                Err(e) => {
                    eprintln!("Error parsing TOML: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Error reading file: {:?}\n{:?}", file, e);
            }
        }

        ColorScheme::default()
    }

    fn load_matches(file: PathBuf) -> Vec<BjjMatch> {
        match fs::read_to_string(&file) {
            Ok(contents) => match toml::from_str::<Matches>(&contents) {
                Ok(matches) => {
                    return matches.matches;
                }
                Err(e) => {
                    eprintln!("Error parsing TOML: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Error reading file: {:?}\n{:?}", file, e);
            }
        }

        BjjMatch::sample_matches()
    }
}

impl egui_multiwin::multi_window::CommonEventHandler<AppCommon, WindowEvent> for AppCommon {
    fn process_event(
        &mut self,
        event: WindowEvent,
    ) -> Vec<egui_multiwin::multi_window::NewWindowRequest<AppCommon>> {
        let mut windows_to_create = vec![];

        match event {
            WindowEvent::NewScorePanel => windows_to_create.push(
                score_window::ScoreWindow::request("Score Window".to_string()),
            ),
            WindowEvent::None => {}
        }
        windows_to_create
    }
}
