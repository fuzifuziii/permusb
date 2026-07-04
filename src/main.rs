use std::io::{self, BufRead, Write};
use std::process::Command;

#[derive(Debug)]
struct USBDevice {
    id: String,
    vendor_id: String,
    product: String,
}

fn parse_lsusb_output(output: &str) -> Vec<USBDevice> {
    let mut devices = Vec::new();

    for line in output.lines() {
        // Example line: Bus 001 Device 002: ID 8087:0a2a Intel Corp.
        if line.contains("ID ") {
            let parts: Vec<&str> = line.split_whitespace().collect();

            // Look for the position of ID in the string
            if let Some(id_index) = parts.iter().position(|&r| r == "ID") {
                if id_index + 1 < parts.len() {
                    let id_pair = parts[id_index + 1];

                    // Split 8087:0a2a by colon to extract the VendorID
                    if let Some(colon_index) = id_pair.find(':') {
                        let vendor_id = &id_pair[..colon_index];
                        // Combine all remaining words into the product name
                        let product_name = parts[id_index + 2..].join(" ");

                        devices.push(USBDevice {
                            id: id_pair.to_string(),
                            vendor_id: vendor_id.to_string(),
                            product: product_name,
                        });
                    }
                }
            }
        }
    }
    devices
}

fn select_device(devices: &[USBDevice]) -> Option<&USBDevice> {
    if devices.is_empty() {
        return None;
    }

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        // Clear screen
        print!("\x1B[2J\x1B[H");
        println!("Select a USB device (enter the number):\n");

        for (i, dev) in devices.iter().enumerate() {
            println!("  [{}] {} (Vendor ID: {}, Product: {})", i + 1, dev.id, dev.vendor_id, dev.product);
        }
        println!("\n[q] Quit\n");
        print!("Enter device number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        handle.read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            return None;
        }

        // Try to parse the input into a number
        if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= devices.len() {
                return Some(&devices[choice - 1]);
            }
        }

        println!("\nInvalid input. Press Enter to continue...");
        let mut tmp = String::new();
        handle.read_line(&mut tmp).unwrap();
    }
}

fn main() {
    // Execute lsusb
    let output = match Command::new("lsusb").output() {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Error executing lsusb: {}", e);
            std::process::exit(1);
        }
    };

    let output_str = String::from_utf8_lossy(&output.stdout);
    let devices = parse_lsusb_output(&output_str);

    if devices.is_empty() {
        println!("No USB devices found.");
        std::process::exit(1);
    }

    // Prompt user for selection
    let selected_device = match select_device(&devices) {
        Some(dev) => dev,
        None => {
            println!("Selection canceled.");
            std::process::exit(0);
        }
    };

    println!(
        "\nSelected device: {} (Vendor ID: {}, Product: {})",
        selected_device.id, selected_device.vendor_id, selected_device.product
    );

    // Create udev rule
    let rule = format!(
        "KERNEL==\"hidraw*\", ATTRS{{idVendor}}==\"{}\", MODE=\"0666\"",
        selected_device.vendor_id
    );
    let rules_file_path = "/etc/udev/rules.d/99-custom-hid.rules";

    // Write the rule using sudo tee
    let echo_cmd = format!("echo '{}' | sudo tee {}", rule, rules_file_path);
    let write_status = Command::new("sh")
        .arg("-c")
        .arg(&echo_cmd)
        .output();

    match write_status {
        Ok(out) if out.status.success() => {}
        Ok(out) => {
            eprintln!("Error writing rule: {}", String::from_utf8_lossy(&out.stderr));
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute write command: {}", e);
            std::process::exit(1);
        }
    }

    // Reload udev rules
    let reload_cmd = "sudo udevadm control --reload-rules && sudo udevadm trigger";
    let reload_status = Command::new("sh")
        .arg("-c")
        .arg(reload_cmd)
        .output();

    match reload_status {
        Ok(out) if out.status.success() => {}
        Ok(out) => {
            eprintln!("Error reloading udev: {}", String::from_utf8_lossy(&out.stderr));
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute udev reload command: {}", e);
            std::process::exit(1);
        }
    }

    println!("udev rule successfully added and reloaded.");
}
