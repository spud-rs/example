// repalce these with a macro?
// spud::load_mods!();
mod app;
mod controller;
mod model;

use app::App;

fn main() {
    App::run()
}
