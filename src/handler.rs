use crate::app::{App, AppResult, BookEditFocus, BookState, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let updated_task = if let Some(mut task) = app.book_edit_state.take() {
        match (key_event.code, task.focus) {
            (KeyCode::Enter, BookEditFocus::Title) => Some(task),
            (_, BookEditFocus::Title) => {
                task.title.input(key_event);
                Some(task)
            }
            (_, BookEditFocus::Author) => {
                task.author.input(key_event);
                Some(task)
            }
            _ => Some(task),
        }
    } else {
        None
    };
    app.book_edit_state = updated_task;
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Remove book
        KeyCode::Char('d') => {
            if app.items.len() != 0 {
                app.remove_json_at_index().expect("Failed to remove");
            }
        }
        KeyCode::Char('a') => {
            app.book_edit_state = Some(BookState::default());
            app.show_popup = !app.show_popup;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.items.len() != 0 {
                app.previous();
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.items.len() != 0 {
                app.next();
            }
        }
        _ => {}
    }
    Ok(())
}
