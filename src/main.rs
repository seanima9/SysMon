mod app;
mod cli;
mod config;
mod widgets;

use app::app_main;

fn main() {
    let run = app_main();
    if let Err(e) = run {
        eprintln!("Error: {}", e);
    }
}
