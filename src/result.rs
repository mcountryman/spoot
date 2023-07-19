use crate::conf::Conf;
use crate::icons;
use egui::{Color32, Sense, TextStyle, Ui, WidgetText};

#[derive(Clone)]
pub struct SearchResult {
  icon: char,
  text: String,
  action: Option<SearchAction>,
}

impl SearchResult {
  /// Sets the icon of the search results.
  pub fn icon(mut self, icon: char) -> Self {
    self.icon = icon;
    self
  }

  /// Sets the text of the search result.
  pub fn text<S: Into<String>>(mut self, text: S) -> Self {
    self.text = text.into();
    self
  }

  /// Sets the action to open the given url.
  pub fn open_with<S: Into<String>>(mut self, url: S) -> Self {
    self.action = Some(SearchAction::OpenUrl(url.into()));
    self
  }

  /// Draws the search result.
  pub fn show(&self, ui: &mut Ui, conf: &Conf) {
    let available = ui.available_size();

    let mut desired_size = egui::vec2(0.0, 0.0);

    let available_width = ui.available_width() - conf.style.margin.x;
    let icon = WidgetText::from(self.icon.to_string()).into_galley(ui, None, available_width, TextStyle::Body);

    let available_width = ui.available_width() - conf.style.margin.x * 2. - icon.size().x;
    let text = WidgetText::from(&self.text).into_galley(ui, None, available_width, TextStyle::Body);

    desired_size += conf.style.margin;
    desired_size += icon.size();
    desired_size += text.size();
    desired_size.x = available.x;

    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    let visuals = if self.action.is_some() {
      ui.style().interact(&response)
    } else {
      &ui.style().visuals.widgets.noninteractive
    };

    ui.painter().rect(rect, 0.0, Color32::TRANSPARENT, visuals.bg_stroke);

    let rect = rect.shrink2(conf.style.margin);

    let icon_pos = ui.layout().align_size_within_rect(icon.size(), rect);
    let rect = rect.shrink2(egui::vec2(icon.size().x, 0.));
    let rect = rect.shrink2(conf.style.margin);
    let text_pos = ui.layout().align_size_within_rect(text.size(), rect);

    icon.paint_with_visuals(ui.painter(), icon_pos.min, visuals);
    text.paint_with_visuals(ui.painter(), text_pos.min, visuals);

    if response.clicked() {
      match &self.action {
        Some(SearchAction::OpenUrl(url)) => open::that(url).ok().unwrap_or_default(),
        Some(SearchAction::CopyToClipboard(_)) => {}
        None => {}
      }
    }
  }
}

impl Default for SearchResult {
  fn default() -> Self {
    Self {
      icon: icons::SHROOM,
      text: String::new(),
      action: None,
    }
  }
}

#[derive(Clone)]
enum SearchAction {
  OpenUrl(String),
  CopyToClipboard(String),
}
