use std::{
    env,
    process::{Command, Stdio},
    time::Duration,
};

use serde_json::json;

fn get_volume_info() -> (f32, bool) {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
        .expect("Failed to run wpctl");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = output_str.trim().split_whitespace().collect();
    let volume = parts
        .get(1)
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(0.0);
    let muted = output_str.contains("[MUTED]");

    (volume, muted)
}

fn set_volume(new_volume: f32) {
    let clamped = new_volume.clamp(0.0, 1.0);
    let _ = Command::new("wpctl")
        .args(["set-volume", "@DEFAULT_AUDIO_SINK@", &clamped.to_string()])
        .stdout(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .ok();
}

fn toggle_mute() {
    Command::new("wpctl")
        .args(["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"])
        .spawn()
        .ok();
}
fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle click / scroll events
    if args.len() > 1 {
        let (volume, _) = get_volume_info();

        match args[1].as_str() {
            "click-left" => toggle_mute(),
            "scroll-up" => set_volume(volume + 0.05),
            "scroll-down" => set_volume(volume - 0.05),
            _ => {}
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    let (volume, muted) = get_volume_info();
    let percent = (volume * 100.0).round() as u8;
    /*
    F057Evolume-high
    F057Fvolume-low
    F0580volume-medium
    F075Evolume-minus
    F075Fvolume-mute
    F0581volume-off
    F075Dvolume-plus
    F1120volume-source
    F0E08volume-variant-off
    F1121volume-vibrate
    */
    let volume_high = "\u{F057E}";
    let volume_low = "\u{F057F}";
    let volume_medium = "\u{F0580}";
    let volume_mute = "\u{F075F}";
    let volume_off = "\u{F0581}";

    let state = match get_volume_info() {
        (_, muted) if muted => volume_mute,
        (volume, _) if volume == 0.0 => volume_off,
        (volume, _) if volume <= 0.25 => volume_low,
        (volume, _) if volume <= 0.5 => volume_medium,
        (_, _) => volume_high,
    };

    let tooltip = if muted {
        format!("Volume: {}% [MUTED]", percent)
    } else {
        format!("Volume: {}%", percent)
    };

    let output = json!({
        "text": state,
        "tooltip": tooltip,
        "class": if muted { "muted" } else { "volume" }
    });

    println!("{}", output);
}
