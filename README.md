## Waybar Volume Widget (Rust + wpctl)
A custom Waybar module written in Rust that:

- Shows the current volume with a Unicode icon
- Displays tooltip with exact volume percentage
- Responds to:
    - **Left click** to toggle mute
    - **Scroll up** to increase volume by 5%
    - **Scroll down** to decrease volume by 5%

## Features

- Uses wpctl to interface with PipeWire
- Icons change based on volume level:
    - 🔇 Muted / 0%
    - 🔈 1–25%
    - 🔉 26–50%
    - 🔊 51–100%
- Outputs JSON in Waybar-compatible format

## Requirements
- [`wpctl`](https://pipewire.pages.freedesktop.org/wireplumber/) (part of WirePlumber)
- Rust toolchain (`cargo`)

## Installation

Install using Cargo:

```bash
cargo install waybar-volume
```
Ensure `~/.cargo/bin` is in your `PATH`.

## Waybar Configuration

Add the following to your Waybar config (`~/.config/waybar/config.jsonc`):

```json
{
  "custom/volume": {
    "exec": "waybar-volume",
    "on-click": "waybar-volume click-left",
    "on-scroll-up": "waybar-volume scroll-up",
    "on-scroll-down": "waybar-volume scroll-down",
    "interval": 1,
    "return-type": "json"
  }
}
```
## Example Output

When volume is 45%, not muted:
```bash
🔉
```

When muted:

```bash
🔇
```

Tooltip will show:

```bash
Volume: 45%
```

## Troubleshooting
- Ensure `wpctl` is installed and `@DEFAULT_AUDIO_SINK@` is available.
- Run `wayland-volume` manually if no output appears in Waybar.

## License
MIT - do whatever you want.
