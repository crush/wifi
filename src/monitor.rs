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

pub fn signal() {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    loop {
        let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
            .arg("-I")
            .output()
            .expect("failed");

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
            execute!(stdout, SetForegroundColor(Color::DarkGrey), Print("not connected\n"), ResetColor).unwrap();
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
                SetForegroundColor(Color::Cyan),
                Print(format!("{}\n\n", ssid)),
                SetForegroundColor(color),
                Print(format!("{} ", bars)),
                ResetColor,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{}dBm\n", rssi)),
                Print(format!("snr {}dB\n", snr)),
                ResetColor
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
        .args(["-s", "-o", "/dev/null", "-w", "%{size_download}", "https://speed.cloudflare.com/__down?bytes=104857600"])
        .output()
        .expect("failed");

    let elapsed = start.elapsed().as_secs_f64();
    let bytes: f64 = String::from_utf8_lossy(&output.stdout).parse().unwrap_or(0.0);
    let mbps = (bytes * 8.0) / (elapsed * 1_000_000.0);

    let color = match mbps as i32 {
        m if m >= 500 => style::green,
        m if m >= 100 => style::cyan,
        _ => style::dim,
    };
    color(&format!("{:.0}", mbps));
    style::dim(" Mbps\n");
}
