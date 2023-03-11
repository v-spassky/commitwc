use std::io::{self, Write};
use std::process::{Command, Stdio};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

/// This is a simple program that allows you to write
/// a commit message and see its length as you type.
/// It also copies the message to the clipboard.
fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;

    let mut ch_count: usize = 0;
    let mut input_text = String::new();

    write!(stdout, "\rEnter commit message: {}", ch_count)?;
    stdout.flush()?;

    for c in stdin.keys() {
        match c? {
            Key::Char('\n') | Key::Ctrl('c') => break,
            Key::Char(c) => {
                ch_count += 1;
                input_text.push(c);
                write!(
                    stdout,
                    "{}\rEnter commit message: {} {} {}",
                    clear::CurrentLine,
                    input_text,
                    ch_count,
                    cursor::Hide,
                )?;
            }
            Key::Backspace => {
                if ch_count > 0 {
                    ch_count -= 1;
                    input_text.pop();
                    write!(
                        stdout,
                        "{}\rEnter commit message: {} {} {}",
                        clear::CurrentLine,
                        input_text,
                        ch_count,
                        cursor::Hide,
                    )?;
                }
            }
            _ => (),
        }
        stdout.flush()?;
    }

    write!(stdout, "\n\r",)?;

    let echo_cmd = Command::new("echo")
        .arg("-n")
        .arg(input_text)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to launch echo command.");

    let _xclip_cmd = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(echo_cmd.stdout.unwrap())
        .spawn()
        .expect("Failed to launch xclip command.");

    Ok(())
}
