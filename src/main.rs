// repalce these with a macro?
// spud::load_mods!();
mod controller;
mod model;
mod app;
use app::App;

fn main() {
    App::run()
}
