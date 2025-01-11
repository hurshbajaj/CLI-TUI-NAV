mod config;

use std::{io, env, fs, path::PathBuf, time::Duration, path};
use std::path::Path;
use crossterm::*;
use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};
use event::Event;
use tui::style::Modifier;

fn main() ->Result<(), Box<dyn std::error::Error> >
{
    let mut changeDir = false;
    let mut breakNow = false;

    let mut i =0;
    let mut state = ListState::default();

    let mut out = io::stdout();
    terminal::enable_raw_mode().unwrap();
    out.execute(terminal::Clear(terminal::ClearType::All))?;
    out.execute(cursor::Hide)?;

    let backend = CrosstermBackend::new(&mut out);
    let mut terminal = Terminal::new(backend)?;

    let mut focusDir = PathBuf::from(config::get_config().get("spawn").unwrap().to_string());
    let mut entries:Vec<ListItem> = changeEntries(focusDir.clone())?;
    'outer:loop{
        if breakNow{
            break 'outer;
        }

        terminal.draw( |f| {
            let chunks = Layout::default().direction(Direction::Vertical).constraints( [Constraint::Percentage(90), Constraint::Percentage(10)].as_ref(),).split(f.size());

            let Ex = List::new(entries.iter().cloned().collect::<Vec<ListItem>>()) //iter "borrows" and clone gives ownership of clone.
                .block(Block::default().borders(Borders::ALL)
                    .title("CLI Navigation")).style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Green)
                    .add_modifier(Modifier::BOLD)).highlight_symbol(" #  ");

            let binding = focusDir.clone();
            let input_displ = Paragraph::new(binding.to_string_lossy())
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title("Path  "));

            if event::poll(Duration::from_millis(100)).expect("polling failed") {
                if let Ok(Event::Key(KeyEvent { code, .. })) = event::read(){
                    match code{
                         KeyCode::Enter => {
                             let current_dir = focusDir.clone();
                             let raw_entry = entriesRaw(&current_dir, i).unwrap();
                             focusDir.push(Path::new(&raw_entry));

                             breakNow = true;

                             changeDir = true;
                         }
                        KeyCode::Esc => {
                            breakNow = true;
                        }
                        KeyCode::Right => {

                            let current_dir = focusDir.clone();
                            let raw_entry = entriesRaw(&current_dir, i).unwrap();
                            focusDir.push(Path::new(&raw_entry));
                            entries = changeEntries(focusDir.clone()).expect("");
                            i = 0
                        }
                        KeyCode::Left => {
                            focusDir.pop();
                            entries = changeEntries(focusDir.clone()).expect("");
                            i = 0;
                        }
                        KeyCode::Up => {
                            i -= 1
                        }
                        KeyCode::Down => {
                            i += 1
                        }
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

    let mut outPost = io::stdout();

    outPost.execute(terminal::Clear(terminal::ClearType::All)).expect("");
    terminal::disable_raw_mode().unwrap();
    outPost.execute(cursor::MoveTo(0, 0));
    outPost.execute(cursor::Show).expect("");

    // if changeDir{
    //     std::process::Command::new("sh").arg("-c").arg(format!("cd \"{}\" ", focusDir.display())).output().expect("Hmmm; Smth went wrong /:");
    //     env::set_current_dir(format!("cd \"{}\" ", focusDir.display())).expect("Oops");
    // }
    if changeDir{
        // echo "Hello, Clipboard!" | clip.exe
        std::process::Command::new("sh").arg("-c").arg(format!("echo cd '\"{}\"' | clip.exe", focusDir.display())).output().expect("Hmmm; Smth went wrong /:");
    }

    Ok(())
}
fn changeEntries(dir:PathBuf) -> Result<Vec<ListItem<'static>>, Box<dyn std::error::Error>>{
    let entries= fs::read_dir(&dir)?.into_iter().map(|x| x.unwrap().file_name().to_string_lossy().to_string()).collect::<Vec<String>>().iter().map(|entry| ListItem::new(entry.to_string())).collect::<Vec<ListItem>>();

    Ok(entries)
}
fn entriesRaw(dir:&PathBuf, i:usize) -> Result<String, Box<dyn std::error::Error>>{
    let entries= fs::read_dir(&dir)?.into_iter().map(|x| x.unwrap().file_name().to_string_lossy().to_string()).collect::<Vec<String>>();

    Ok(entries.get(i).expect("").to_string())
}