use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor, ResetColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::process::Command;

pub fn signal() {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    loop {
        let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
            .arg("-I")
            .output()
            .expect("failed to get signal");

        let info = String::from_utf8_lossy(&output.stdout);
        let mut rssi = 0i32;
        let mut noise = 0i32;
        let mut ssid = String::new();

        for line in info.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let val = parts[1].trim();
                match key {
                    "agrCtlRSSI" => rssi = val.parse().unwrap_or(0),
                    "agrCtlNoise" => noise = val.parse().unwrap_or(0),
                    "SSID" => ssid = val.to_string(),
                    _ => {}
                }
            }
        }

        execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        if ssid.is_empty() {
            execute!(stdout, Print("not connected\n")).unwrap();
        } else {
            let snr = rssi - noise;
            let bars = match rssi {
                r if r >= -50 => "████",
                r if r >= -60 => "███░",
                r if r >= -70 => "██░░",
                r if r >= -80 => "█░░░",
                _ => "░░░░",
            };
            let color = match rssi {
                r if r >= -50 => Color::Green,
                r if r >= -70 => Color::Yellow,
                _ => Color::Red,
            };
            execute!(
                stdout,
                Print(format!("{}\n\n", ssid)),
                SetForegroundColor(color),
                Print(format!("{} {}dBm\n", bars, rssi)),
                ResetColor,
                Print(format!("noise {}dBm  snr {}dB\n", noise, snr)),
                Print("\nq to quit")
            ).unwrap();
        }

        stdout.flush().unwrap();

        if event::poll(std::time::Duration::from_secs(1)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if matches!(key.code, KeyCode::Esc | KeyCode::Char('q')) {
                    break;
                }
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
}

pub fn speed() {
    let start = std::time::Instant::now();
    let output = Command::new("curl")
        .args(["-s", "-o", "/dev/null", "-w", "%{size_download}", "http://speedtest.tele2.net/1MB.zip"])
        .output()
        .expect("failed");

    let elapsed = start.elapsed().as_secs_f64();
    let bytes: f64 = String::from_utf8_lossy(&output.stdout).parse().unwrap_or(0.0);
    let mbps = (bytes * 8.0) / (elapsed * 1_000_000.0);
    println!("{:.1} Mbps", mbps);
}
