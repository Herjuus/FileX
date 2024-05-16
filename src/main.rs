mod app;
mod ui;

use std::io;

use app::{App, CurrentScreen};
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal};
use ui::ui;


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {

// FUNKSJONER:
// ny fil
// slett fil
// Åpne fil
// Favorites (kanskje)
// Help screen

    loop {
        let _ = app.filesystem.update_directories();
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up => app.move_up(1),
                    KeyCode::Down => app.move_down(1),
                    KeyCode::Left => app.move_up(5),
                    KeyCode::Right => app.move_down(5),
                    KeyCode::Backspace => app.filesystem.go_back(),
                    KeyCode::Enter => app.filesystem.open_go_forward(),
                    KeyCode::Char('c') => app.copy_path(),
                    _ => {}
                }
                _ => {}
            }
        }
    }

    Ok(())
}