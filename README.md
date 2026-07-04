# permusb

> A simple interactive CLI utility written in Rust that permanently grants user access to USB HID devices by automatically generating the required **udev** rules.

No more running your HID applications with `sudo`.

---

## ✨ Features

- Automatically detects connected USB devices using `lsusb`
- Interactive terminal menu for selecting a device
- Creates the appropriate `udev` rule automatically
- Grants read/write permissions (`0666`) to selected HID devices
- Reloads and applies `udev` rules immediately
- Written in Rust

---

## Requirements

- Rust toolchain (`cargo`)
- `lsusb` (`usbutils` package)
- `sudo` privileges (only required when writing udev rules)

### Install `lsusb`

#### Arch Linux

```bash
sudo pacman -S usbutils
```

#### Ubuntu / Debian

```bash
sudo apt install usbutils
```

#### Fedora

```bash
sudo dnf install usbutils
```

---

## Installation

Clone the repository:

```bash
git clone https://github.com/fuzifuziii/permusb.git
cd permusb
```

Run directly with Cargo:

```bash
cargo run
```

---

## Global Installation

Build the release version:

```bash
cargo build --release
```

Install the binary:

```bash
sudo cp target/release/permusb /usr/local/bin/
```

Now it can be executed from anywhere:

```bash
permusb
```

---

## How It Works

1. Scans connected USB devices using `lsusb`.
2. Displays an interactive selection menu.
3. Extracts the selected device's Vendor ID.
4. Generates a `udev` rule like:

```udev
KERNEL=="hidraw*", ATTRS{idVendor}=="YOUR_VENDOR_ID", MODE="0666"
```

5. Saves the rule to:

```
/etc/udev/rules.d/99-custom-hid.rules
```

6. Reloads and triggers `udev`:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

The device becomes accessible without requiring `sudo`.

---

## Notes

- Only USB **HID** devices are supported.
- Existing rule files may be overwritten depending on implementation.
- Administrator privileges are only needed when modifying system `udev` rules.
