pub const VERSION: &[usize] = &[0, 1, 0];
pub const PRELUDE_FILENAME: &str = ".atom-prelude";
pub const HISTORY_FILENAME: &str = ".atom-history";
use std::cmp::min;

pub use asciicolor::Colorize;

mod env;
pub use env::{CWD, Environment, REPORT, PROMPT, INCOMPLETE_PROMPT};

mod value;
pub use value::{Value, Size};

mod error;
pub use error::Error;

use comment::atom::strip;

use lalrpop_util::{lalrpop_mod, ParseError};
lalrpop_mod!(parser);
use parser::ProgramParser;

pub fn parse(code: impl ToString) -> Result<Value, Error> {
    let code = code.to_string();
    let code = match strip(&code) {
        Ok(s) => s,
        Err(_) => code.clone(),
    };

    match ProgramParser::new().parse(&code) {
        Ok(val) => Ok(val),
        Err(e) => Err(Error::SyntaxError(format!("\n{}", format_error(&code, e))))
    }
}

pub type SyntaxError<'a, T> = ParseError<usize, T, &'a str>;

/// This formats an error properly given the line, the `unexpected` token as a string,
/// the line number, and the column number of the unexpected token.
pub fn make_error(line: &str, unexpected: &str, line_number: usize, column_number: usize) -> String {
    // The string used to underline the unexpected token
    let underline = format!(
        "{}^{}",
        " ".repeat(column_number),
        "-".repeat(unexpected.len() - 1)
    );

    // Format string properly and return
    format!(
        "{WS} |
{line_number} | {line}
{WS} | {underline}
{WS} |
{WS} = unexpected `{unexpected}`",
        WS = " ".repeat(line_number.to_string().len()),
        line_number = line_number,
        line = line.bright_yellow().underline(),
        underline = underline,
        unexpected = unexpected.bright_yellow().underline()
    )
}

// Gets the line number, the line, and the column number of the error
pub fn get_line(script: &str, location: usize) -> (usize, String, usize) {
    // Get the line number from the character location
    let line_number = script[..min(location + 1, script.len())].lines().count();
    // Get the line from the line number
    let line = match script.lines().nth(line_number - 1) {
        Some(line) => line,
        None => {
            if let Some(line) = script.lines().last() {
                line
            } else {
                ""
            }
        }
    };

    // Get the column number from the location
    let mut column = {
        let mut current_column = 0;
        // For every character in the script until the location of the error,
        // keep track of the column location
        for ch in script[..location].chars() {
            if ch == '\n' {
                current_column = 0;
            } else {
                current_column += 1;
            }
        }
        current_column
    };

    // Trim the beginning of the line and subtract the number of spaces from the column
    let trimmed_line = line.trim_start();
    column -= (line.len() - trimmed_line.len()) as i32;

    (line_number, String::from(trimmed_line), column as usize)
}

/// This is used to take an LALRPOP error and convert
/// it into a nicely formatted error message
pub fn format_error<T: core::fmt::Debug>(script: &str, err: SyntaxError<T>) -> String {
    match err {
        SyntaxError::InvalidToken { location } => {
            let (line_number, line, column) = get_line(script, location);
            make_error(
                &line,
                &(script.as_bytes()[location] as char).to_string(),
                line_number,
                column,
            )
        }
        SyntaxError::UnrecognizedEOF { location, .. } => {
            let (line_number, line, _) = get_line(script, location);
            make_error(&line, "EOF", line_number, line.len())
        }
        SyntaxError::UnrecognizedToken { token, .. } => {
            // The start and end of the unrecognized token
            let start = token.0;
            let end = token.2;

            let (line_number, line, column) = get_line(script, start);
            let unexpected = &script[start..end];
            make_error(&line, unexpected, line_number, column)
        }
        SyntaxError::ExtraToken { token } => {
            // The start and end of the extra token
            let start = token.0;
            let end = token.2;

            let (line_number, line, column) = get_line(script, start);
            let unexpected = &script[start..end];

            make_error(&line, unexpected, line_number, column)
        }
        SyntaxError::User { error } => format!(
            "  |\n? | {}\n  | {}\n  |\n  = unexpected compiling error",
            error,
            format!("^{}", "-".repeat(error.len() - 1))
        ),
    }
}
