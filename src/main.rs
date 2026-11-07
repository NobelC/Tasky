mod app;
mod event;
mod tui;

use {app::App, tui::run};

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| run(&mut app, terminal))
}
