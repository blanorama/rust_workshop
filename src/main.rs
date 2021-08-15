use std::time::Duration;

use gamestate::GameState;
use termion;
use termion::raw::IntoRawMode;
use termion::terminal_size;

use std::io::{Read, Write};

mod direction;
mod gamestate;
mod point;

fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = std::io::stdout()
        .into_raw_mode()
        .expect("Can't enable terminal raw mode, exiting...");

    let mut stdin = termion::async_stdin().bytes();

    write!(
        &mut stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .ok();
    let (width, height) = terminal_size().expect("Can't get terminal size, exiting....");

    let mut gamestate = GameState::new(height.into(), width.into());

    loop {
        write!(
            &mut stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .ok();
        stdout.flush().unwrap();

        let key = stdin.next();

        match key {
            Some(Ok(b'c')) => {
                write!(
                    &mut stdout,
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
                gamestate.draw(x.into(), y.into(), &mut stdout);
            }
        }

        stdout.flush().unwrap();
        std::thread::sleep(Duration::from_millis(33))
    }
}
