use app::App;
use conf::Conf;
use eframe::{AppCreator, NativeOptions};

mod app;
mod conf;
mod icons;
mod result;
mod search;

fn main() -> Result<(), anyhow::Error> {
  let conf = Conf::from_filesystem()?.unwrap_or_default();
  let app: AppCreator = Box::new(move |ctx| Box::new(App::new(conf, ctx)));
  let options = NativeOptions {
    centered: true,
    decorated: false,

    transparent: true,
    ..Default::default()
  };

  eframe::run_native("spoot", options, app).unwrap();

  Ok(())
}
