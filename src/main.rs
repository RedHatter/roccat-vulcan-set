extern crate hidapi;

use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
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

    // Colors are in blocks of 12 keys (2 columns). Color parts are sorted by color e.g. the red
    // values for all 12 keys are first then come the green values etc.
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

fn parse_color<'a, I>(mut args: I, key: &str) -> Result<Color, UserError>
where
    I: Iterator<Item = &'a str>,
{
    let value = ok_or_user_error!(args.next(), "Color value missing for {}", key);

    let without_prefix = value.trim_start_matches("0x").trim_start_matches("#");
    if without_prefix.len() == 6 {
        let hex = or_user_error!(
            u32::from_str_radix(&without_prefix, 16),
            "Invalid HEX color {} for {}",
            &without_prefix,
            key
        );
        Ok(Color {
            r: u8::try_from((hex >> 16) & 0xff).unwrap(),
            g: u8::try_from((hex >> 8) & 0xff).unwrap(),
            b: u8::try_from(hex & 0xff).unwrap(),
        })
    } else {
        let green = ok_or_user_error!(args.next(), "Green value missing for {}", key);
        let blue = ok_or_user_error!(args.next(), "Blue value missing for {}", key);
        Ok(Color {
            r: or_user_error!(
                value.parse::<u8>(),
                "Invalid red value {} for {}",
                &value,
                key
            ),
            g: or_user_error!(
                green.parse::<u8>(),
                "Invalid green value {} for {}",
                &green,
                key
            ),
            b: or_user_error!(
                blue.parse::<u8>(),
                "Invalid blue value {} for {}",
                &blue,
                key
            ),
        })
    }
}

fn set_map<'a, I>(map: &mut [Color; RV_NUM_KEYS], mut args: I) -> Result<(), UserError>
where
    I: Iterator<Item = &'a str>,
{
    while let Some(key) = args.next() {
        let color = parse_color(&mut args, &key)?;
        let normalized = key.to_uppercase();
        if normalized == "ALL" {
            for key in map.iter_mut() {
                *key = color
            }
        } else {
            let key_code = ok_or_user_error!(
                util::parse_key_name(normalized.trim_start_matches("KEY_")),
                "Invalid key code {}",
                &key
            );
            map[key_code] = color;
        }
    }

    Ok(())
}

fn run() -> Result<(), UserError> {
    let led_device = open_device()?;

    let mut map: [Color; RV_NUM_KEYS] = [Color { r: 0, g: 0, b: 0 }; RV_NUM_KEYS];

    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        loop {
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Error reading stdin");
            if buf.is_empty() {
                break;
            }

            set_map(&mut map, buf.split_whitespace())?;
            send_led_map(&led_device, &map);
        }
    } else {
        set_map(&mut map, args.iter().map(|s| s.as_ref()))?;
        send_led_map(&led_device, &map);
    }

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
