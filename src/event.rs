use crate::app::App;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

pub fn handle_key_event(app: &mut App) -> io::Result<()> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => key_code(app, key),
        _ => todo!(),
    };
    Ok(())
}

fn key_code(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.set_new_state(),
        KeyCode::Char('p') => app.set_new_title(),
        //{} evita que se arroje un panic, todo!() arroja un panic
        _ => {}
    }
}
