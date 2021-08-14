use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use termion::terminal_size;

fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = std::io::stdout()
        .into_raw_mode()
        .expect("Can't enable terminal raw mode, exiting...");

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();
    let (width, height) = terminal_size().expect("Can't get terminal size, exiting....");

    for y in 0..height {
        for x in 0..width {
            //Use print because terminal made newlines automatically
            print!("▇");
        }
    }
}
