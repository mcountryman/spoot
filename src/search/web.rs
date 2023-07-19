use crate::conf::Conf;
use crate::icons;
use crate::result::SearchResult;
use std::sync::mpsc;

pub fn search(_: &Conf, query: &str, tx: mpsc::Sender<SearchResult>) {
  tx.send(
    SearchResult::default()
      .icon(icons::SPIDER_WEB)
      .text(query)
      .open_with(format!("https://google.com/search?q={query}")),
  )
  .ok();
}
