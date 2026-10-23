# Sonard

`sonard` is a lightweight Linux daemon built on top of [`pipewire-native`](https://docs.rs/pipewire-native) and [`pipewire-native-spa`](https://docs.rs/pipewire-native-spa). It monitors the state of sound cards and microphones (activation, mute, plug, unplug) in real time and emits desktop notification instructions.

## Features

* **Real-Time Monitoring**: Tracks audio sinks and sources dynamically via PipeWire registry events.
* **Low Overhead**: Built in Rust around a finite state machine architecture designed to run without unnecessary heap allocations.
* **Customizable Notifications**: Fully configurable icons and event triggers using a clean TOML configuration file.
* **Systemd Integration**: Runs seamlessly as a user-level background daemon managed by systemd.

---

## Configuration

`sonard` automatically searches for its configuration file (`config.toml`) in the following XDG paths:
1. `$XDG_DATA_HOME/sonard/config.toml` (typically `~/.local/share/sonard/config.toml`)
2. `$XDG_CONFIG_HOME/sonard/config.toml` (typically `~/.config/sonard/config.toml`)

### Example Configuration (`config.toml`)

Below is an example of a configuration file customizing icons for audio output (sinks) and input (microphones):

```toml
[sinks]
current = "audio-volume-high"
plugged = "audio-headset"
unplugged = "audio-headset-symbolic"

[mic]
unmuted = "microphone-sensitivity-high"
muted = "microphone-sensitivity-muted"
plugged = "audio-input-microphone"
unplugged = "microphone-disabled"
```

---

## Systemd Service Setup

An example service file (`sonard.service`) is provided to manage the daemon with `systemd`:

```ini
[Unit]
Description=Sonard - PipeWire Audio State Tracker
After=pipewire.service
Requires=pipewire.service

[Service]
Type=simple
ExecStart=%h/.local/bin/sonard
Restart=on-failure
RestartSec=3

[Install]
WantedBy=default.target
```

### Installation Steps

1. Copy or build the `sonard` executable into `~/.local/bin/sonard`.
2. Save the `sonard.service` configuration file to `~/.config/systemd/user/sonard.service`.
3. Enable and start the service:

```bash
systemctl --user daemon-reload
systemctl --user enable --now sonard.service
```

---

## License

Licensed under the MIT License. See the `LICENSE` file for more information.