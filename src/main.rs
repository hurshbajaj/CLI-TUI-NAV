use std::io;
use std::fs;
use std::env;
use std::os::linux::raw::stat;
use std::path::PathBuf;
//use ratatui::*;
use crossterm::*;
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState},
    Frame,
    layout::Rect,
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};
use tui::style::Modifier;

fn main() ->Result<(), Box<dyn std::error::Error> >
{
    let mut out = io::stdout();
    terminal::enable_raw_mode().unwrap();
    out.execute(terminal::Clear(terminal::ClearType::All))?;
    out.execute(cursor::Hide)?;

    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;

    let mut focusDir = env::current_dir().unwrap();
    let mut entries:Vec<ListItem> = changeEntries(&focusDir)?;

    loop{
        terminal.draw( |f| {
            let mut chunks = Layout::default().direction(Direction::Vertical).constraints( [Constraint::Percentage(90), Constraint::Percentage(10)].as_ref(),).split(f.size());

            let Ex = List::new(entries.iter().cloned().collect::<Vec<ListItem>>()) //iter "borrows" and clone gives ownership of clone.
                .block(Block::default().borders(Borders::ALL)
                    .title("CLI Navigation")).style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Green)
                    .add_modifier(Modifier::BOLD)).highlight_symbol(" #  ");

            let input_displ = Paragraph::new(focusDir.to_string_lossy())
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title("Path  "));

            let mut state = ListState::default();
            state.select(Some(3));

            f.render_widget(input_displ, chunks[1]);
            f.render_stateful_widget(Ex, chunks[0], &mut state);

        })?;
    }
}
fn changeEntries(dir:&PathBuf) -> Result<Vec<ListItem>, Box<dyn std::error::Error>>{
    let entries= fs::read_dir(&dir)?.into_iter().map(|x| x.unwrap().file_name().to_string_lossy().to_string()).collect::<Vec<String>>().iter().map(|entry| ListItem::new(entry.to_string())).collect::<Vec<ListItem>>();

    Ok(entries)
}
