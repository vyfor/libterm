use libterm::{Event, KeyCode, Mode, MoveTo, Print, Terminal};

fn main() -> std::io::Result<()> {
    let mut term = Terminal::open()?;
    term.enable_raw_mode()?;
    term.push_mode(Mode::AltScreen)?;

    term.emit(MoveTo::new(0, 0));
    term.emit(Print("hello world! press Escape to exit"));
    term.flush()?;

    while let Some(event) = term.poll(None)? {
        if let Event::Key(key) = event
            && key.code == KeyCode::Esc
        {
            break;
        }
    }

    Ok(())
}
