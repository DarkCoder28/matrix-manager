# Matrix Manager Client

## Material Requirements
- Raspberry Pi 4/5 or Zero 2 W
- [Adafruit RGB Matrix Bonnet](https://www.adafruit.com/product/3211)
- Adafruit RGB Matrix (Such As: [this one](https://www.adafruit.com/product/2278), [this one](https://www.adafruit.com/product/2276), or [another similar one from here](https://www.adafruit.com/search?q=RGB+LED+Matrix&c=327))
- A [power supply](https://www.adafruit.com/product/1466) for the Matrix Bonnet
- A (short) jumper wire to bridge pin 4 to 18 on the bonnet

## Environment Setup
- Most of the OS setup tips can be found in the [hzeller/rpi-rgb-led-matrix](https://github.com/hzeller/rpi-rgb-led-matrix?tab=readme-ov-file#troubleshooting) repo.
  - Instead of disabling NTP, as is recomended in the repo, it is better to change the configs so it effectively only runs on boot.
  - You can do this by modifying `/etc/systemd/timesyncd.conf` with the following options under the `[Time]` header
    - `RootDistanceMaxSec=3600`
    - `PollIntervalMinSec=31536000`
    - `PollIntervalMaxSec=31536000`
- Make sure to use the [Improving Flicker tip](https://github.com/hzeller/rpi-rgb-led-matrix?tab=readme-ov-file#improving-flicker) as it is used by default by this program.
- The [Improving CPU use tip](https://github.com/hzeller/rpi-rgb-led-matrix?tab=readme-ov-file#improving-flicker) is a must for running on Pi Zero 2W.

### Service Setup
- If you are not using the default `pi` user, you may have to modify the supervisor config files to reflect that
- Put the `client` binary in the home folder of your user... you could also probably use the `/opt` folder and modify the supervisor configs for that as well.
- I recommend using [Supervisor](https://github.com/Supervisor/supervisor) to run the matrix client, as it also creates a web GUI that allows the services it manages to be stoped, started, and restarted on command. It also allows you to read the logs from the web UI as well.
  - To install it, run `apt-get install supervisor -y`
  - Replace the default config in `/etc/supervisor/supervisord.conf` with the one in [service-configs/supervisor](/client/service-configs/supervisor/supervisord.conf)
  - Add the premade `matrix-client.conf` config to `/etc/supervisor/conf.d/` (found in [service-configs/supervisor](/client/service-configs/supervisor/matrix-client.conf))
  - Enable the `supervisord` service with `sudo systemctl enable --now supervisord`
- I also recommend using systemd units to set up a restart timer that will make sure everything stays working more stably
  - Put `daily-restart.service` from [service-configs/systemd](/client/service-configs/systemd/daily-restart.service) in the `/etc/systemd/system/` folder
  - Put `daily-restart.timer` from [service-configs/systemd](/client/service-configs/systemd/daily-restart.timer) in the `/etc/systemd/system/` folder
  - Run `sudo systemctl daemon-reload`
  - Run `sudo systemctl enable --now daily-restart.timer`

## Compilation Prerequisites
To build this project, you need:
- The `stable-x86_64-unknown-linux-gnu` Rust toolchain
- The `armv7-unknown-linux-gnueabihf` Rust target (for a 32bit client)
- The `aarch64-unknown-linux-gnu` Rust target (for a 64bit client)
- Cross-compiling the client for raspberry pi requires `arm-linux-gnueabihf-gcc` (`arm-linux-gnueabi-gcc` on Arch) for a 32bit client
- Cross-compiling the client for raspberry pi requires `aarch64-linux-gnu-gcc` (`aarch64-linux-gnu-gcc` on Arch) for a 64bit client
- [cross-rs](https://github.com/cross-rs/cross) (A tool for easily cross-compiling rust projects)

## Compilation
### 32-bit Raspberry Pi OS
- `cd` into the client folder (this one)
- Run the `cross build --target armv7-unknown-linux-gnueabihf --release` command
- The compiled binary should be in `../target/armv7-unknown-linux-gnueabihf/release/` and called `client`
### 64-bit Raspberry Pi OS
- `cd` into the client folder (this one)
- Run the `cross build --target aarch64-unknown-linux-gnu --release` command
- The compiled binary should be in `../target/aarch64-unknown-linux-gnu/release/` and called `client`

# Developing
Run the client with `cargo run` to compile the development debug binary launch it. You can pass arguments as needed via `cargo run -- --arg1 val`

# License
This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file for details.

# Contributing
Contributions are welcome! Please feel free to make bug reports, open pull requests or suggest features.

# Disclaimer
This is a hobby project, and I don't have the time or resources to devote myself to it full-time. Please bear with me as I continue to develop and improve Matrix Manager.
