extern crate termion;
extern crate tui;
extern crate yaml_rust;

use termion::event::Key;
//use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;
use std::io;
use tui::style::{Color, Style};

mod events;

pub fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = events::Events::new();

    let mut i: usize = 0;

    loop {
        i += 1;
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(16),
                        Constraint::Percentage(16),
                        Constraint::Percentage(16),
                        Constraint::Percentage(16),
                        Constraint::Percentage(16),
                        Constraint::Percentage(16),
                    ]
                    .as_ref(),
                )
                .split(size);
            let mut day: usize = 0;
            for chunk in chunks {
                let vchunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                            Constraint::Percentage(16),
                        ]
                        .as_ref(),
                    )
                    .split(chunk);
                for vchunk in vchunks {
                    day += 1;
                    let title = format!("January {:?}", day);
                    let text = [
                        Text::styled("hello world\n\n", Style::default().fg(Color::Red)),
                    ];
                    let b = Block::default()
                        .title(&title)
                        .border_style(Style::default().fg(Color::Cyan))
                        .borders(Borders::RIGHT | Borders::TOP);
                    Paragraph::new(text.iter())
                        .block(b)
                        .alignment(Alignment::Left)
                        .render(&mut f, vchunk);
                }
            }
        })?;

        match events.next() {
            Ok(ev) => match ev {
                events::Event::Input(input) => match input {
                    Key::Char('q') => break,
                    Key::Char(input) => i = 0,
                    Key::Ctrl('l') => {
                        terminal.clear();
                    }
                    _ => {}
                },
                events::Event::Tick => {
                    //println!("ticking");
                }
            },
            Err(_) => {
                println!("Error");
            }
        }
    }
    Ok(())
}
