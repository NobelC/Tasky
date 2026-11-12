mod app;
mod backend;
mod event;
mod tui;
mod utilities;

use {crate::backend::project::Project, app::App, tui::run};

fn main() -> std::io::Result<()> {
    let mut data = backend::project_manager::ProjectManager::new();
    let mut app = App::new();
    ratatui::run(|terminal| run(&mut app, terminal))
}
