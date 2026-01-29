use crossterm::{execute, style::{Color, Print, SetForegroundColor, ResetColor}};
use std::io::{self, Write};

pub fn cyan(text: &str) {
    let mut out = io::stdout();
    execute!(out, SetForegroundColor(Color::Cyan), Print(text), ResetColor).unwrap();
    out.flush().unwrap();
}

pub fn green(text: &str) {
    let mut out = io::stdout();
    execute!(out, SetForegroundColor(Color::Green), Print(text), ResetColor).unwrap();
    out.flush().unwrap();
}

pub fn dim(text: &str) {
    let mut out = io::stdout();
    execute!(out, SetForegroundColor(Color::DarkGrey), Print(text), ResetColor).unwrap();
    out.flush().unwrap();
}

pub fn red(text: &str) {
    let mut out = io::stderr();
    execute!(out, SetForegroundColor(Color::Red), Print(text), ResetColor).unwrap();
    out.flush().unwrap();
}
