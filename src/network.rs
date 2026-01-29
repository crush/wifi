use crate::style;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor, ResetColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::process::Command;

pub fn current() {
    let output = Command::new("networksetup")
        .args(["-getairportnetwork", "en0"])
        .output()
        .expect("failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("not associated") {
        style::dim("not connected\n");
    } else if let Some(name) = stdout.strip_prefix("Current Wi-Fi Network: ") {
        style::cyan(name.trim());
        println!();
    } else {
        style::dim("not connected\n");
    }
}

pub fn list() {
    let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        .arg("-s")
        .output()
        .expect("failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() <= 1 {
        style::dim("no networks\n");
        return;
    }

    let networks: Vec<String> = lines[1..]
        .iter()
        .filter_map(|line| {
            let name = line.chars().take(32).collect::<String>().trim().to_string();
            if name.is_empty() { None } else { Some(name) }
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    if networks.is_empty() {
        style::dim("no networks\n");
        return;
    }

    let mut selected = 0;
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    loop {
        execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        for (i, network) in networks.iter().enumerate() {
            if i == selected {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Cyan),
                    Print(format!("  {}\n", network)),
                    ResetColor
                ).unwrap();
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!("  {}\n", network)),
                    ResetColor
                ).unwrap();
            }
        }

        stdout.flush().unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if selected > 0 { selected -= 1; }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if selected < networks.len() - 1 { selected += 1; }
                }
                KeyCode::Enter => {
                    terminal::disable_raw_mode().unwrap();
                    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
                    connect(&networks[selected]);
                    return;
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    terminal::disable_raw_mode().unwrap();
                    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
                    return;
                }
                _ => {}
            }
        }
    }
}

pub fn connect(name: &str) {
    let output = Command::new("networksetup")
        .args(["-setairportnetwork", "en0", name])
        .output()
        .expect("failed");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if stderr.contains("password") || stdout.contains("password") || !output.status.success() {
        style::dim("password: ");
        io::stdout().flush().unwrap();

        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();
        let password = password.trim();

        let output = Command::new("networksetup")
            .args(["-setairportnetwork", "en0", name, password])
            .output()
            .expect("failed");

        if output.status.success() && output.stderr.is_empty() {
            style::green("connected ");
            style::cyan(name);
            println!();
        } else {
            style::red("failed\n");
        }
    } else {
        style::green("connected ");
        style::cyan(name);
        println!();
    }
}

pub fn off() {
    Command::new("networksetup")
        .args(["-setairportpower", "en0", "off"])
        .output()
        .expect("failed");
    style::dim("off\n");
}

pub fn on() {
    Command::new("networksetup")
        .args(["-setairportpower", "en0", "on"])
        .output()
        .expect("failed");
    style::green("on\n");
}

pub fn pass(name: Option<&str>) {
    let network = match name {
        Some(n) => n.to_string(),
        None => {
            let output = Command::new("networksetup")
                .args(["-getairportnetwork", "en0"])
                .output()
                .expect("failed");
            let stdout = String::from_utf8_lossy(&output.stdout);
            match stdout.strip_prefix("Current Wi-Fi Network: ") {
                Some(n) => n.trim().to_string(),
                None => {
                    style::dim("not connected\n");
                    return;
                }
            }
        }
    };

    let output = Command::new("security")
        .args(["find-generic-password", "-ga", &network])
        .output()
        .expect("failed");

    let stderr = String::from_utf8_lossy(&output.stderr);
    for line in stderr.lines() {
        if let Some(pass) = line.strip_prefix("password: ") {
            let pass = pass.trim_matches('"');
            style::cyan(pass);
            println!();
            return;
        }
    }
    style::dim("not found\n");
}
