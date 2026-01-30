use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::process::Command;

pub fn current() {
    if let Some(name) = get_ssid_fast() {
        println!("{}", name);
        return;
    }
    if let Some(ip) = get_ip() {
        println!("{}", ip);
    } else {
        println!("not connected");
    }
}

pub fn name() {
    if let Some(name) = get_ssid_fast() {
        println!("{}", name);
        return;
    }
    if let Some(name) = get_ssid_slow() {
        println!("{}", name);
    } else if get_ip().is_some() {
        println!("connected");
    } else {
        println!("not connected");
    }
}

fn get_ip() -> Option<String> {
    let output = Command::new("ipconfig")
        .args(["getifaddr", "en0"])
        .output()
        .ok()?;
    let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if ip.is_empty() { None } else { Some(ip) }
}

fn get_ssid_fast() -> Option<String> {
    let output = Command::new("networksetup")
        .args(["-getairportnetwork", "en0"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Some(name) = stdout.strip_prefix("Current Wi-Fi Network: ") {
        let name = name.trim();
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    None
}

fn get_ssid_slow() -> Option<String> {
    let output = Command::new("system_profiler")
        .args(["SPAirPortDataType"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for (i, line) in stdout.lines().enumerate() {
        if line.contains("Current Network Information:") {
            if let Some(next) = stdout.lines().nth(i + 1) {
                let name = next.trim().trim_end_matches(':');
                if !name.is_empty() && !name.contains("Network Type") {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

fn get_current_network() -> Option<String> {
    get_ssid_fast().or_else(get_ssid_slow)
}

pub fn list() {
    let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        .arg("-s")
        .output()
        .expect("failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() <= 1 {
        println!("no networks");
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
        println!("no networks");
        return;
    }

    let mut selected = 0;
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    loop {
        execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        for (i, network) in networks.iter().enumerate() {
            if i == selected {
                print!("> {}\r\n", network);
            } else {
                print!("  {}\r\n", network);
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
        print!("password: ");
        io::stdout().flush().unwrap();

        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();
        let password = password.trim();

        let output = Command::new("networksetup")
            .args(["-setairportnetwork", "en0", name, password])
            .output()
            .expect("failed");

        if output.status.success() && output.stderr.is_empty() {
            println!("connected {}", name);
        } else {
            eprintln!("failed");
        }
    } else {
        println!("connected {}", name);
    }
}

pub fn off() {
    Command::new("networksetup")
        .args(["-setairportpower", "en0", "off"])
        .output()
        .expect("failed");
    println!("off");
}

pub fn on() {
    Command::new("networksetup")
        .args(["-setairportpower", "en0", "on"])
        .output()
        .expect("failed");
    println!("on");
}

pub fn pass(name: Option<&str>) {
    let network = match name {
        Some(n) => n.to_string(),
        None => match get_current_network() {
            Some(n) => n,
            None => {
                println!("not connected");
                return;
            }
        },
    };

    let output = Command::new("security")
        .args(["find-generic-password", "-ga", &network])
        .output()
        .expect("failed");

    let stderr = String::from_utf8_lossy(&output.stderr);
    for line in stderr.lines() {
        if let Some(pass) = line.strip_prefix("password: ") {
            let pass = pass.trim_matches('"');
            println!("{}", pass);
            return;
        }
    }
    println!("not found");
}
