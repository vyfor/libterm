# libterm

a modern cross-platform terminal library for rust.

the project is still early, and both name and api are subject to change.

## examples

```rust
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
```

### acknowledgements

- https://vt100.net/emu/dec_ansi_parser
- https://vt100.net/docs/vt510-rm/
- https://ecma-international.org/publications-and-standards/standards/ecma-48/
- https://invisible-island.net/xterm/ctlseqs/ctlseqs.html
- https://www.linusakesson.net/programming/tty/
- https://www.columbia.edu/kermit/ascii.html
