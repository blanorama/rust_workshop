use std::time::Duration;

use gamestate::GameState;
use termion;
use termion::raw::IntoRawMode;
use termion::terminal_size;

use std::io::{Read, Write};

mod gamestate;

fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = std::io::stdout()
        .into_raw_mode()
        .expect("Can't enable terminal raw mode, exiting...");

    let mut stdin = termion::async_stdin().bytes();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .ok();
    let (width, height) = terminal_size().expect("Can't get terminal size, exiting....");

    let mut gamestate = GameState::new(height.into(), width.into());

    loop {
        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        )
        .ok();
        stdout.flush().unwrap();

        let key = stdin.next();

        match key {
            Some(Ok(b'c')) => {
                write!(
                    stdout,
                    "{}{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1),
                    termion::cursor::Show
                )
                .ok();
                return;
            }
            Some(Ok(x)) => {
                gamestate.update_gamestate(x.into());
            }
            _ => {
                gamestate.update_gamestate('c');
            }
        }

        for x in 0..width {
            for y in 0..height {
                if gamestate.draw(x.into(), y.into()) {
                    write!(stdout, "{}\u{2B1B}", termion::cursor::Goto(x + 1, y + 1)).ok();
                }
            }
        }

        stdout.flush().unwrap();
        std::thread::sleep(Duration::from_millis(10))
    }
}
