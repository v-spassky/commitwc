use std::io::{self, Write};
use std::process::{Command, Stdio};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

const CURSOR: char = '<';

/// This is a simple program that allows you to write
/// a commit message and see its length as you type.
/// It also copies the message to the clipboard.
fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;

    let mut ch_count: usize = 0;
    let mut cursor_pos: usize = 0;
    let mut input_text = String::from(CURSOR);

    write!(
        stdout,
        "\rEnter commit message: {} {}",
        input_text, ch_count
    )?;
    stdout.flush()?;

    for c in stdin.keys() {
        match c? {
            Key::Char('\n') | Key::Ctrl('c') => break,
            Key::Char(c) => {
                input_text.insert(cursor_pos, c);
                ch_count += 1;
                cursor_pos += 1;
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
                    input_text.remove(cursor_pos - 1);
                    ch_count -= 1;
                    cursor_pos -= 1;
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
            Key::Left => {
                if ch_count > 0 && cursor_pos > 0 {
                    let rem = input_text.remove(cursor_pos - 1);
                    input_text.insert(cursor_pos, rem);
                    cursor_pos -= 1;
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
            Key::Right => {
                if ch_count < input_text.len() && cursor_pos < input_text.len() - 1 {
                    let rem = input_text.remove(cursor_pos + 1);
                    input_text.insert(cursor_pos, rem);
                    cursor_pos += 1;
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
    write!(stdout, "{}", cursor::Show)?;
    input_text.remove(cursor_pos);

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
