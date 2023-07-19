use crate::conf::Conf;
use crate::search::Searcher;
use eframe::egui::{Context, Frame, TextEdit, Ui};
use eframe::epaint::Color32;
use egui::{Key, LayerId, Margin, ScrollArea};

pub struct App {
  conf: Conf,
  searcher: Searcher,
  has_had_focus: bool,
}

impl App {
  /// Creates a new [App].
  pub fn new(conf: Conf, ctx: &eframe::CreationContext<'_>) -> Self {
    ctx.egui_ctx.set_style(conf.style());

    Self {
      conf: conf.clone(),
      searcher: Searcher::new(conf),
      has_had_focus: false,
    }
  }
}

impl App {
  /// Handles auto hide on focus loss.
  fn handle_focus(&mut self, frame: &mut eframe::Frame) {
    let focused = frame.info().window_info.focused;
    if focused {
      self.has_had_focus = true;
    }

    if !focused && self.has_had_focus {
      frame.close();
    }
  }

  /// Handles search results.
  fn handle_search(&mut self) {
    self.searcher.poll();
  }

  /// Handles window size.
  fn handle_size(&mut self, frame: &mut eframe::Frame) {
    let style = &self.conf.style;
    let monitor_size = frame
      .info()
      .window_info
      .monitor_size
      .unwrap_or(egui::vec2(1000., 1000.));

    let desired_size = egui::vec2(
      (monitor_size.x * style.window_scale_factor.x).min(style.window_max_size.x),
      (monitor_size.y * style.window_scale_factor.y).min(style.window_max_size.y),
    );

    frame.set_centered();
    frame.set_window_size(desired_size);
  }

  /// Draw the search bar.
  fn draw_search(&mut self, ui: &mut Ui) {
    let id = "search".into();

    // 🔍

    TextEdit::singleline(&mut self.searcher)
      .id(id)
      .margin(self.conf.style.margin)
      .hint_text(&self.conf.i18n.search_placeholder)
      .interactive(true)
      .min_size(egui::vec2(ui.available_width(), 0.))
      .show(ui);

    ui.memory_mut(|m| {
      if m.focus().is_none() {
        m.request_focus(id);
      }
    })
  }

  fn draw_results(&self, ui: &mut Ui) {
    if self.searcher.results.is_empty() {
      return;
    }

    let style = ui.style();

    Frame::default()
      .fill(style.visuals.extreme_bg_color)
      .stroke(style.visuals.widgets.inactive.bg_stroke)
      .rounding(style.visuals.widgets.inactive.rounding)
      .show(ui, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
          for (_, result) in self.searcher.results.iter().enumerate() {
            result.show(ui, &self.conf);
          }

          ui.set_min_width(ui.available_width());
        });
      });
  }
}

impl eframe::App for App {
  fn clear_color(&self, _: &eframe::egui::Visuals) -> [f32; 4] {
    Color32::TRANSPARENT.to_normalized_gamma_f32()
  }

  fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
    self.handle_size(frame);
    self.handle_focus(frame);
    self.handle_search();

    let mut ui = Ui::new(
      ctx.clone(),
      LayerId::background(),
      "spoot".into(),
      ctx.available_rect(),
      ctx.screen_rect(),
    );

    ui.input(|i| i.key_down(Key::Escape).then(|| frame.close()));

    Frame::default()
      .fill(Color32::TRANSPARENT)
      .outer_margin(Margin::same(1.))
      .show(&mut ui, |ui| {
        ui.vertical(|ui| {
          self.draw_search(ui);
          ui.add_space(self.conf.style.margin.y);
          self.draw_results(ui);
        });
      });
  }
}
