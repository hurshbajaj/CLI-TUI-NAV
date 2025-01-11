use std::io;
//use ratatui::*;
use crossterm::*;
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};

fn main()->Result<(), Box<dyn std::error::Error> >
{
    let mut out = io::stdout();
    terminal::enable_raw_mode().unwrap();
    out.execute(terminal::Clear(terminal::ClearType::All))?;
    out.execute(cursor::Hide)?;

    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();

    loop{
        terminal.draw( |f| {
            let mut chunks = Layout::default().direction(Direction::Vertical).constraints( [Constraint::Percentage(90), Constraint::Percentage(10)].as_ref(),).split(f.size());

            let input_paragraph = Paragraph::new(input.as_ref())
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title("$  "));

            f.render_widget(input_paragraph, chunks[1]);

        })?;
    }
} 
