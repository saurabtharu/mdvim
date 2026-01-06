mod app;
mod renderer;
mod syntax;
mod ui;

use std::io;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use ui::render_ui;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| render_ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,

                KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.toggle_tree();
                }

                KeyCode::Char('t') => {
                    app.toggle_tree();
                }

                KeyCode::Char('g') => {
                    if app.last_key == Some('g') {
                        app.scroll_to_top();
                    } else {
                        app.last_key = Some('g');
                    }
                }

                KeyCode::Char('G') => {
                    app.scroll_to_bottom();
                }

                KeyCode::Char('j') | KeyCode::Down if app.show_tree => {
                    app.next_file();
                }

                KeyCode::Char('k') | KeyCode::Up if app.show_tree => {
                    app.prev_file();
                }

                KeyCode::Char('j') | KeyCode::Down if !app.show_tree => {
                    app.scroll_down(1);
                }

                KeyCode::Char('k') | KeyCode::Up if !app.show_tree => {
                    app.scroll_up(1);
                }

                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.scroll_down(10);
                }

                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.scroll_up(10);
                }

                KeyCode::PageDown => {
                    app.scroll_down(20);
                }

                KeyCode::PageUp => {
                    app.scroll_up(20);
                }

                KeyCode::Home => {
                    app.scroll_to_top();
                }

                KeyCode::End => {
                    app.scroll_to_bottom();
                }

                KeyCode::Enter if app.show_tree => {
                    app.open_selected_file();
                }

                KeyCode::Char('o') if app.show_tree => {
                    app.open_selected_file();
                }

                _ => {
                    app.last_key = None;
                }
            }
        } else if let Event::Mouse(mouse) = event::read()? {
            match mouse.kind {
                MouseEventKind::ScrollDown => {
                    app.scroll_down(3);
                }
                MouseEventKind::ScrollUp => {
                    app.scroll_up(3);
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
