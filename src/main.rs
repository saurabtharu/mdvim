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

        let evt = event::read()?;

        match evt {
            Event::Key(key) => {
                let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

                match key.code {
                    KeyCode::Char('q') => break,

                    // Toggle file tree visibility
                    KeyCode::Char('n') if ctrl => {
                        app.toggle_tree();
                    }

                    KeyCode::Char('t') => {
                        app.toggle_tree();
                    }

                    // Focus movement: Ctrl+h (tree), Ctrl+l (preview)
                    KeyCode::Char('h') if ctrl => {
                        app.focus_tree();
                    }

                    KeyCode::Char('l') if ctrl => {
                        app.focus_preview();
                    }

                    // Ctrl+ww (like vim: toggle focus between panes)
                    KeyCode::Char('w') if ctrl => {
                        if app.last_key == Some('w') {
                            app.toggle_focus();
                            app.last_key = None;
                        } else {
                            app.last_key = Some('w');
                        }
                    }

                    // Vim-style gg / G navigation in preview
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

                    // j / k depending on which pane is focused
                    KeyCode::Char('j') | KeyCode::Down => {
                        if app.show_tree && matches!(app.focused_pane, app::FocusedPane::FileTree) {
                            app.next_file();
                        } else {
                            app.scroll_down(1);
                        }
                    }

                    KeyCode::Char('k') | KeyCode::Up => {
                        if app.show_tree && matches!(app.focused_pane, app::FocusedPane::FileTree) {
                            app.prev_file();
                        } else {
                            app.scroll_up(1);
                        }
                    }

                    // Faster scrolling
                    KeyCode::Char('d') if ctrl => {
                        app.scroll_down(10);
                    }

                    KeyCode::Char('u') if ctrl => {
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

                    // Open file from tree (only when visible)
                    KeyCode::Enter if app.show_tree => {
                        app.open_selected_file();
                    }

                    KeyCode::Char('o') if app.show_tree => {
                        app.open_selected_file();
                    }

                    // Resize tree: Ctrl+Left shrink, Ctrl+Right grow
                    KeyCode::Left if ctrl => {
                        app.decrease_tree_width();
                    }

                    KeyCode::Right if ctrl => {
                        app.increase_tree_width();
                    }

                    _ => {
                        app.last_key = None;
                    }
                }
            }
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => {
                    app.scroll_down(3);
                }
                MouseEventKind::ScrollUp => {
                    app.scroll_up(3);
                }
                _ => {}
            },
            _ => {}
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
