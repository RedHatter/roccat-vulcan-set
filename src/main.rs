extern crate hidapi;

use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::exit;

use hidapi::{HidApi, HidDevice};

#[macro_use]
mod util;

const RV_PRODUCTS: [u16; 2] = [0x3098, 0x307a];
const RV_VENDOR: u16 = 0x1e7d;
const RV_LED_INTERFACE: i32 = 3;
const RV_NUM_KEYS: usize = 144;

#[derive(Debug)]
pub struct UserError {
    message: String,
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for UserError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn open_device() -> Result<HidDevice, UserError> {
    let api = HidApi::new().expect("Error in HidApi");
    let led_device_info = ok_or_user_error!(
        api.devices()
            .into_iter()
            .find(|o| o.interface_number == RV_LED_INTERFACE
                && o.vendor_id == RV_VENDOR
                && RV_PRODUCTS.contains(&o.product_id)),
        "No LED device found"
    );
    println!("LED interface at USB path {:#?}", led_device_info.path);

    Ok(led_device_info
        .open_device(&api)
        .expect("Error opening led device"))
}

fn send_led_map(device: &HidDevice, map: &[Color; RV_NUM_KEYS]) {
    let mut hwmap: [u8; 444] = [0; 444];

    for (i, color) in map.iter().enumerate() {
        let offset = ((i / 12) * 36) + (i % 12);

        hwmap[offset] = color.r;
        hwmap[offset + 12] = color.g;
        hwmap[offset + 24] = color.b;
    }

    let (slice, hwmap) = hwmap.split_at(60);

    let mut workbuf: [u8; 65] = [0; 65];
    workbuf[1..5].copy_from_slice(&[0xa1, 0x01, 0x01, 0xb4]);
    workbuf[5..65].copy_from_slice(&slice);
    device.write(&workbuf).expect("Error writing to led device");

    for bytes in hwmap.chunks(64) {
        workbuf[1..65].copy_from_slice(bytes);
        device.write(&workbuf).expect("Error writing to led device");
    }
}

fn build_map() -> Result<[Color; RV_NUM_KEYS], UserError> {
    // Initialize all keys to OFF (0, 0, 0)
    let mut map: [Color; RV_NUM_KEYS] = [Color { r: 0, g: 0, b: 0 }; RV_NUM_KEYS];

    let mut args = env::args();
    args.next(); // Skip exec path

    while let Some(key) = args.next() {
        let key_code = ok_or_user_error!(
            util::parse_key_name(&key.to_uppercase().trim_start_matches("KEY_")),
            "Invalid key code {}",
            &key
        );
        let value = ok_or_user_error!(args.next(), "Color value missing for {}", &key);

        let without_prefix = value.trim_start_matches("0x").trim_start_matches("#");
        if without_prefix.len() == 6 {
            let z = or_user_error!(
                u32::from_str_radix(&without_prefix, 16),
                "Invalid HEX color {} for {}",
                &without_prefix,
                &key
            );
            map[key_code] = Color {
                r: u8::try_from((z >> 16) & 0xff).unwrap(),
                g: u8::try_from((z >> 8) & 0xff).unwrap(),
                b: u8::try_from(z & 0xff).unwrap(),
            };
        } else {
            let green = ok_or_user_error!(args.next(), "Green value missing for {}", &key);
            let blue = ok_or_user_error!(args.next(), "Blue value missing for {}", &key);
            map[key_code] = Color {
                r: or_user_error!(
                    value.parse::<u8>(),
                    "Invalid red value {} for {}",
                    &value,
                    &key
                ),
                g: or_user_error!(
                    green.parse::<u8>(),
                    "Invalid green value {} for {}",
                    &green,
                    &key
                ),
                b: or_user_error!(
                    blue.parse::<u8>(),
                    "Invalid blue value {} for {}",
                    &blue,
                    &key
                ),
            };
        }
    }

    Ok(map)
}

fn run() -> Result<(), UserError> {
    let map = build_map()?;
    let led_device = open_device()?;
    send_led_map(&led_device, &map);

    Ok(())
}

fn main() {
    exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err.description());
            1
        }
    });
}
