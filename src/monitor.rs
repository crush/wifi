use std::process::Command;

pub fn signal() {
    let ip = Command::new("ipconfig")
        .args(["getifaddr", "en0"])
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if s.is_empty() { None } else { Some(s) }
        });

    if ip.is_none() {
        println!("not connected");
        return;
    }

    let (rssi, noise) = get_signal_fast();
    if rssi == 0 {
        println!("connected");
        return;
    }

    let snr = rssi - noise;
    let bars = match rssi {
        r if r >= -50 => "████",
        r if r >= -60 => "███░",
        r if r >= -70 => "██░░",
        r if r >= -80 => "█░░░",
        _ => "░░░░",
    };

    println!("{} {}dBm snr {}dB", bars, rssi, snr);
}

fn get_signal_fast() -> (i32, i32) {
    let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        .arg("-I")
        .output();

    if let Ok(output) = output {
        let info = String::from_utf8_lossy(&output.stdout);
        let mut rssi = 0i32;
        let mut noise = 0i32;

        for line in info.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let val = parts[1].trim();
                match key {
                    "agrCtlRSSI" => rssi = val.parse().unwrap_or(0),
                    "agrCtlNoise" => noise = val.parse().unwrap_or(0),
                    _ => {}
                }
            }
        }
        return (rssi, noise);
    }

    (0, 0)
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
    println!("{:.0} Mbps", mbps);
}
