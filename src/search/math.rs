use crate::conf::Conf;
use crate::icons;
use crate::result::SearchResult;
use std::sync::mpsc;

pub fn search(_: &Conf, query: &str, tx: mpsc::Sender<SearchResult>) {
  if let Ok(val) = evalexpr::eval(query) {
    tx.send(
      SearchResult::default()
        .icon(icons::CALCULATOR)
        .text(format!("{query} = {val}")),
    )
    .ok();
  }
}
