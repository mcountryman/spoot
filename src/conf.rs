use anyhow::Context;
use directories::ProjectDirs;
use egui::{FontId, Style, TextStyle, Vec2};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::{env, fs};

/// Spoot configuration object.
#[derive(Clone, Serialize, Deserialize)]
pub struct Conf {
  pub i18n: I18nConf,
  pub style: StyleConf,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct I18nConf {
  /// The text to display in the search bar.
  pub search_placeholder: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StyleConf {
  /// The maximum size of the window.
  pub window_max_size: Vec2,
  /// The scale factor to apply to monitor size to get the window size.
  pub window_scale_factor: Vec2,

  /// The margins to use for all widgets.
  pub margin: Vec2,
  /// The standard font size to use.
  pub font_size: f32,
}

impl Conf {
  /// Loads the [Conf] from the file system.
  pub fn from_filesystem() -> Result<Option<Self>, anyhow::Error> {
    for dir in Self::get_search_dirs() {
      let path = dir.join("spoot.toml");
      if path.exists() {
        return Some(Self::from_path(path)).transpose();
      }
    }

    Ok(None)
  }

  /// Loads the [Conf] from the given path.
  pub fn from_path<P: AsRef<Path> + Debug>(path: P) -> Result<Self, anyhow::Error> {
    let toml = fs::read_to_string(&path).with_context(|| format!("Failed to read conf file {:?}", path))?;

    toml::from_str(&toml).with_context(|| format!("Failed to parse conf file {:?}", path))
  }

  /// Gets the [egui::Style] from the [Conf].
  pub fn style(&self) -> Style {
    use egui::FontFamily::{Monospace, Proportional};

    let StyleConf { font_size, .. } = self.style;

    Style {
      text_styles: [
        (TextStyle::Small, FontId::new(9.0, Proportional)),
        (TextStyle::Body, FontId::new(font_size, Proportional)),
        (TextStyle::Button, FontId::new(font_size, Proportional)),
        (TextStyle::Heading, FontId::new(32.0, Proportional)),
        (TextStyle::Monospace, FontId::new(font_size, Monospace)),
      ]
      .into(),
      ..Default::default()
    }
  }

  /// Gets a list of candidate directories to search for the conf file.
  fn get_search_dirs() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(dir) = env::current_dir() {
      candidates.push(dir);
    }

    if let Some(dirs) = ProjectDirs::from("vin", "maar", "spoot") {
      candidates.push(dirs.config_dir().into());
    }

    candidates
  }
}

impl Default for Conf {
  fn default() -> Self {
    Self {
      style: StyleConf {
        window_max_size: egui::vec2(650., 1000.),
        window_scale_factor: egui::vec2(0.5, 0.5),

        font_size: 24.0,
        margin: egui::vec2(15.0, 15.0),
      },
      i18n: I18nConf {
        search_placeholder: "Search...".into(),
      },
    }
  }
}
