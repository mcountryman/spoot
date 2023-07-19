use crate::conf::Conf;
use crate::result::SearchResult;
use crates_io::Registry;
use curl::easy::Easy;
use once_cell::sync::Lazy;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct CratesResolver {
  state: Arc<Mutex<Option<ResolverState>>>,
}

struct ResolverState {
  query: String,
  sender: mpsc::Sender<SearchResult>,
}

impl CratesResolver {
  fn get() -> &'static Self {
    static GLOBAL: Lazy<CratesResolver> = Lazy::new(|| {
      let state = Arc::new(Mutex::new(None));

      spawn_resolver(state.clone());
      CratesResolver { state }
    });

    &GLOBAL
  }

  fn update(&self, query: &str, tx: mpsc::Sender<SearchResult>) {
    let mut state = self.state.lock().unwrap();

    *state = Some(ResolverState {
      query: query.into(),
      sender: tx,
    });
  }
}

pub fn search(_: &Conf, query: &str, tx: mpsc::Sender<SearchResult>) {
  CratesResolver::get().update(query, tx);
}

fn spawn_resolver(state: Arc<Mutex<Option<ResolverState>>>) -> JoinHandle<()> {
  thread::spawn(move || {
    let mut handle = Easy::new();
    handle.useragent("spoot").ok();

    let mut registry = Registry::new_handle("https://crates.io".into(), None, handle, false);

    loop {
      thread::sleep(Duration::from_millis(300));

      let mut state = state.lock().unwrap();
      let state = match state.take() {
        Some(state) => state,
        None => continue,
      };

      if let Ok((crates, _)) = registry.search(&state.query, 10) {
        for c in crates {
          state
            .sender
            .send(
              SearchResult::default()
                .text(format!("{}@{}", c.name, c.max_version))
                .open_with(format!("https://crates.io/crates/{}/{}", c.name, c.max_version)),
            )
            .ok();
        }
      }
    }
  })
}
