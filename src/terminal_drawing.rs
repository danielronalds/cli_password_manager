use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
    Result,
};
use std::io::stdout;

/// Prints a string followed by a new line and carriage return to the stdout using Crossterm.
/// Works in raw mode
///
/// # Arguments
///
/// * `text` - The text to write to stdout
///
pub fn println<T: ToString>(text: T) -> Result<()> {
    let text = format!("{}\n\r", text.to_string());
    execute!(stdout(), Print(text))?;
    Ok(())
}

/// Prints a string to the stdout using Crossterm. Works in raw mode
///
/// # Arguments
///
/// * `text` - The text to write to stdout
///
pub fn print<T: ToString>(text: T) -> Result<()> {
    execute!(stdout(), Print(text.to_string()))?;
    Ok(())
}

/// Waits for a key event, returning true if the user confirms the action. No by Default
///
/// # Returns
///
/// `true` if the key pressed is 'y' or 'Y'
pub fn get_confirmation() -> Result<bool> {
    match read()? {
        Event::Key(key) => match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => Ok(true),
            _ => Ok(false),
        },
        _ => Ok(false),
    }
}

/// Clears the current line and provides a textbox for the user to type input into
///
/// # Arguments
///
/// * `prompt` - What the textbox prompt should be
/// * `prompt_len` - The length of the prompt
/// * `content` - The initial content of the textfield
///
/// # Returns
///
/// If no errors occured, an Option containing None if the user canceled the operation, or Some
/// containing what the user inputted
pub fn textfield<T: ToString>(
    prompt: T,
    prompt_len: u16,
    content: String,
) -> Result<Option<String>> {
    execute!(stdout(), cursor::Show, cursor::SetCursorStyle::SteadyBlock)?;

    let mut output = content;
    let prompt = prompt.to_string();

    let mut cursor = output.len();

    loop {
        execute!(
            stdout(),
            Clear(ClearType::CurrentLine),
            Print(format!("\r{}{}", prompt, &output)),
            cursor::MoveToColumn(0),
            cursor::MoveRight(prompt_len + (cursor as u16))
        )?;
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char(char) => {
                    output.insert(cursor, char);
                    cursor += 1;
                }
                KeyCode::Backspace => {
                    if !output.is_empty() {
                        output.remove(cursor - 1);
                        cursor = cursor.saturating_sub(1);
                    }
                }
                KeyCode::Esc => return Ok(None),
                KeyCode::Enter => break,
                KeyCode::Left => cursor = cursor.saturating_sub(1),
                KeyCode::Right => {
                    if cursor != output.len() {
                        cursor += 1;
                    }
                }
                _ => (),
            }
        }
    }

    Ok(Some(output))
}
