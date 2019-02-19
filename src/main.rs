extern crate clap;

use clap::{App};

mod ui;

fn main() {
    let matches = App::new("calis").version("0.1.0").get_matches();

    ui::tui::main();
}
