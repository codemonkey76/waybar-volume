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

    let state = match get_volume_info() {
        (_, muted) if muted => "volume-mute",
        (volume, _) if volume == 0.0 => "volume-off",
        (volume, _) if volume <= 0.25 => "volume-low",
        (volume, _) if volume <= 0.5 => "volume-medium",
        (_, _) => "volume-high",
    };

    let tooltip = if muted {
        format!("Volume: {}% [MUTED]", percent)
    } else {
        format!("Volume: {}%", percent)
    };

    let output = json!({
        "text": "",
        "tooltip": tooltip,
        "class": state
    });

    println!("{}", output);
}
