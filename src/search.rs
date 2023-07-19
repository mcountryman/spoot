mod crates;
mod math;
mod web;

use crate::conf::Conf;
use crate::result::SearchResult;
use egui::TextBuffer;
use std::ops::Range;
use std::sync::mpsc;

pub struct Searcher {
  conf: Conf,
  query: String,
  results_rx: Option<mpsc::Receiver<SearchResult>>,

  pub results: Vec<SearchResult>,
}

impl Searcher {
  pub fn new(conf: Conf) -> Searcher {
    Self {
      conf,
      query: String::new(),
      results: Vec::new(),
      results_rx: None,
    }
  }

  /// Polls the search results and fills the results buffer.
  pub fn poll(&mut self) {
    let rx = match self.results_rx.as_ref() {
      Some(rx) => rx,
      None => return,
    };

    while let Ok(result) = rx.try_recv() {
      self.results.push(result);
    }
  }

  pub fn on_query_change(&mut self) {
    let (tx, rx) = mpsc::channel();

    if !self.query.is_empty() {
      math::search(&self.conf, &self.query, tx.clone());
      web::search(&self.conf, &self.query, tx.clone());
      crates::search(&self.conf, &self.query, tx);
    }

    self.results_rx.replace(rx);
    self.results.clear();
  }
}

impl TextBuffer for Searcher {
  fn is_mutable(&self) -> bool {
    true
  }

  fn as_str(&self) -> &str {
    self.query.as_ref()
  }

  fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
    // Get the byte index from the character index
    let byte_idx = self.byte_index_from_char_index(char_index);

    // Then insert the string
    self.query.insert_str(byte_idx, text);
    self.on_query_change();

    text.chars().count()
  }

  fn delete_char_range(&mut self, char_range: Range<usize>) {
    assert!(char_range.start <= char_range.end);

    // Get both byte indices
    let byte_start = self.byte_index_from_char_index(char_range.start);
    let byte_end = self.byte_index_from_char_index(char_range.end);

    // Then drain all characters within this range
    self.query.drain(byte_start..byte_end);
    self.on_query_change();
  }

  fn clear(&mut self) {
    self.query.clear();
    self.on_query_change();
  }

  fn replace(&mut self, text: &str) {
    self.query = text.to_owned();
    self.on_query_change();
  }

  fn take(&mut self) -> String {
    let val = std::mem::take(&mut self.query);
    self.on_query_change();
    val
  }
}
