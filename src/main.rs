use std::{io, env, fs, path::PathBuf, time::Duration, path};
use crossterm::*;
use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState},
    Frame,
    layout::Rect,
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};
use event::Event;
use tui::style::Modifier;

fn main() ->Result<(), Box<dyn std::error::Error> >
{
    let mut breakNow = false;

    let mut i =0;
    let mut state = ListState::default();

    let mut out = io::stdout();
    terminal::enable_raw_mode().unwrap();
    out.execute(terminal::Clear(terminal::ClearType::All))?;
    out.execute(cursor::Hide)?;

    let backend = CrosstermBackend::new(&mut out);
    let mut terminal = Terminal::new(backend)?;

    let mut focusDir = env::current_dir().unwrap();
    let mut entries:Vec<ListItem> = changeEntries(&focusDir)?;
    'outer:loop{
        if breakNow{
            break 'outer;
        }

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

            if event::poll(Duration::from_millis(50)){
                 if let Event::Key(KeyEvent { code, .. }) = event::read(){
                     match code{
                         KeyCode::Enter => {
                             focusDir.push(entries[i].to_string());

                             out.execute(terminal::Clear(terminal::ClearType::All)).expect("");
                             terminal::disable_raw_mode().unwrap();
                             out.execute(cursor::Show).expect("");
                             breakNow = true;

                             env::set_current_dir(focusDir.to_str().unwrap()).unwrap();
                         }
                         KeyCode::Esc => {
                             out.execute(terminal::Clear(terminal::ClearType::All)).expect("");
                             terminal::disable_raw_mode().unwrap();
                             out.execute(cursor::Show).expect("");
                             breakNow = true;
                         }
                         //for left/right, follow~ push into focus dir, re-generate entries
                         //for top/bottom, change <i>
                         _ => {}
                     }
                 }
            }


            state.select(Some(i));

            f.render_widget(input_displ, chunks[1]);
            f.render_stateful_widget(Ex, chunks[0], &mut state);

        })?;
    }

    Ok(())
}
fn changeEntries(dir:&PathBuf) -> Result<Vec<ListItem>, Box<dyn std::error::Error>>{
    let entries= fs::read_dir(&dir)?.into_iter().map(|x| x.unwrap().file_name().to_string_lossy().to_string()).collect::<Vec<String>>().iter().map(|entry| ListItem::new(entry.to_string())).collect::<Vec<ListItem>>();

    Ok(entries)
}
