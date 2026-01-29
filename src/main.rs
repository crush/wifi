use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor, ResetColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(|s| s.as_str()) {
        None => current(),
        Some("list") => list(),
        Some("off") => off(),
        Some("on") => on(),
        Some(name) => connect(name),
    }
}

fn current() {
    let output = Command::new("networksetup")
        .args(["-getairportnetwork", "en0"])
        .output()
        .expect("failed to get network");

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("not associated") {
        println!("not connected");
    } else if let Some(name) = stdout.strip_prefix("Current Wi-Fi Network: ") {
        print!("{}", name.trim());
    } else {
        println!("not connected");
    }
}

fn list() {
    let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        .arg("-s")
        .output()
        .expect("failed to scan networks");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() <= 1 {
        println!("no networks found");
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
        println!("no networks found");
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
                    Print(format!("> {}\n", network)),
                    ResetColor
                ).unwrap();
            } else {
                execute!(stdout, Print(format!("  {}\n", network))).unwrap();
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

fn connect(name: &str) {
    let output = Command::new("networksetup")
        .args(["-setairportnetwork", "en0", name])
        .output()
        .expect("failed to connect");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if stderr.contains("password") || stdout.contains("password") || !output.status.success() {
        print!("password: ");
        io::stdout().flush().unwrap();

        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();
        let password = password.trim();

        let output = Command::new("networksetup")
            .args(["-setairportnetwork", "en0", name, password])
            .output()
            .expect("failed to connect");

        if output.status.success() && output.stderr.is_empty() {
            println!("connected to {}", name);
        } else {
            eprintln!("failed to connect");
        }
    } else {
        println!("connected to {}", name);
    }
}

fn off() {
    Command::new("networksetup")
        .args(["-setairportpower", "en0", "off"])
        .output()
        .expect("failed to turn off wifi");
    println!("wifi off");
}

fn on() {
    Command::new("networksetup")
        .args(["-setairportpower", "en0", "on"])
        .output()
        .expect("failed to turn on wifi");
    println!("wifi on");
}
