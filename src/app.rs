use crate::bjj_match::{BjjMatch, MatchInformation, Matches};
use crate::font_config::FontConfig;
use crate::score_grid::{GridConfig, ScoreGrid};
use crate::{color_scheme::ColorScheme, windows::score_window};
use directories::ProjectDirs;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use crate::flag::Flags;

const QUALIFIER: &str = "au";
const ORGANIZATION: &str = "popplestones";
const COLOR_SCHEME_FILE: &str = "color_scheme.toml";
const MATCHES_FILE: &str = "matches.toml";
const GRID_CONFIG_FILE: &str = "grid_config.toml";
const FONT_CONFIG_FILE: &str = "font_config.toml";

#[derive(Debug)]
pub struct AppCommon {
    pub show_score_window: bool,
    pub show_debug_grid: bool,
    //All application information is stored here.
    pub color_scheme: ColorScheme,
    pub matches: Vec<BjjMatch>,
    pub selected_match: Option<BjjMatch>,
    pub match_info: MatchInformation,
    pub grid_config: GridConfig,
    pub font_config: FontConfig,
    pub score_grids: Option<ScoreGrid>,
    pub flags: Flags,
    pub is_dirty: bool,
    pub new_match: bool,
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
            let matches: Matches = AppCommon::load_item(config_dir.join(MATCHES_FILE));
            return Self {
                show_debug_grid: false,
                show_score_window: false,
                color_scheme: AppCommon::load_item(config_dir.join(COLOR_SCHEME_FILE)),
                matches: matches.matches,
                selected_match: None,
                match_info: MatchInformation::new(5),
                grid_config: AppCommon::load_item(config_dir.join(GRID_CONFIG_FILE)),
                font_config: AppCommon::load_item(config_dir.join(FONT_CONFIG_FILE)),
                score_grids: None,
                flags: Default::default(),
                is_dirty: false,
                new_match: false
            };
        } else {
            eprintln!("Could not find project directory");
        }

        Self {
            show_score_window: false,
            show_debug_grid: false,
            color_scheme: Default::default(),
            matches: vec![],
            selected_match: None,
            match_info: MatchInformation::new(5),
            grid_config: Default::default(),
            font_config: Default::default(),
            score_grids: None,
            flags: Default::default(),
            is_dirty: false,
            new_match: false
        }
    }

    pub fn save(&self) {
        if let Some(config_dir) = AppCommon::get_config_dir() {
            self.save_item(config_dir.join(COLOR_SCHEME_FILE), &self.color_scheme);
            self.save_item(config_dir.join(GRID_CONFIG_FILE), &self.grid_config);
            self.save_item(config_dir.join(COLOR_SCHEME_FILE), &self.color_scheme);
            self.save_item(config_dir.join(FONT_CONFIG_FILE), &self.font_config);

            let matches = Matches {
                matches: self.matches.clone(),
            };

            self.save_item(config_dir.join(MATCHES_FILE), &matches);
        } else {
            eprintln!("Could not find project directory");
        }
    }

    fn load_item<T>(file: PathBuf) -> T
    where
        T: serde::de::DeserializeOwned + Default + Debug,
    {
        match fs::read_to_string(&file) {
            Ok(contents) => match toml::from_str::<T>(&contents) {
                Ok(item) => {
                    return item;
                }
                Err(e) => {
                    eprintln!("Error parsing TOML: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Error reading file: {:?}\n{:?}", file, e);
            }
        }

        Default::default()
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
