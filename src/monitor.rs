use std::process::Command;

pub fn signal() {
    let (ssid, rssi, noise) = get_signal_info();

    if ssid.is_empty() {
        println!("not connected");
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

    println!("{}", ssid);
    println!("{} {}dBm snr {}dB", bars, rssi, snr);
}

fn get_signal_info() -> (String, i32, i32) {
    let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        .arg("-I")
        .output();

    if let Ok(output) = output {
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
        if !ssid.is_empty() {
            return (ssid, rssi, noise);
        }
    }

    let output = Command::new("system_profiler")
        .args(["SPAirPortDataType"])
        .output();

    if let Ok(output) = output {
        let info = String::from_utf8_lossy(&output.stdout);
        let mut ssid = String::new();
        let mut rssi = 0i32;
        let mut noise = 0i32;
        let mut in_current = false;

        for line in info.lines() {
            if line.contains("Current Network Information:") {
                in_current = true;
                continue;
            }
            if in_current && ssid.is_empty() {
                let name = line.trim().trim_end_matches(':');
                if !name.is_empty() && !name.contains("Network Type") {
                    ssid = name.to_string();
                }
            }
            if line.contains("Signal / Noise:") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    let vals: Vec<&str> = parts[1].split('/').collect();
                    if vals.len() == 2 {
                        rssi = vals[0].trim().replace(" dBm", "").parse().unwrap_or(0);
                        noise = vals[1].trim().replace(" dBm", "").parse().unwrap_or(0);
                    }
                }
                break;
            }
        }
        return (ssid, rssi, noise);
    }

    (String::new(), 0, 0)
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
