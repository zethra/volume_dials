extern crate alsa;
extern crate serial;

use std::time::Duration;
use std::path::Path;
use std::thread;

use std::io::prelude::*;
use serial::prelude::*;

use alsa::mixer::{
    Mixer,
    SelemId,
    SelemChannelId,
};

fn main() {
    let serial_port = "/dev/ttyACM0";
    let mixer = match Mixer::new("default", false) {
        Ok(mixer) => mixer,
        Err(_) => {
            println!("Couldn't get alsa mixer");
            return;
        },
    };
    let selem_id = SelemId::new("Master", 0);
    let selem = match mixer.find_selem(&selem_id) {
        Some(selem) => selem,
        None => {
            println!("Couldn't get selem Master");
            return;
        }
    };
    let (_, max) = selem.get_playback_volume_range();

    loop {
        let mut port = match serial::open(serial_port) {
            Ok(port) => port,
            Err(_) => {
                println!("Could not open serial port");
                thread::sleep(Duration::from_secs(1));
                continue;
            },
        };
        if port.reconfigure(&|settings| {
            if settings.set_baud_rate(serial::Baud9600).is_err() { println!("Couldn't set baud rate"); }
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        }).is_err() {
            println!("Couldn't configure serial port");
            thread::sleep(Duration::from_secs(1));
            continue;
        };
        if port.set_timeout(Duration::from_secs(1)).is_err() {
            println!("Couldn't set port timeout");
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        let mut buf: Vec<u8> = vec![0; 7];
        loop {
            if port.read_exact(&mut buf[..]).is_err() {
                println!("Couldn't read from serial port");
                if !Path::new(serial_port).exists() {
                    println!("Serial port closed");
                    thread::sleep(Duration::from_secs(1));
                    break;
                }
            } else {
                if buf[0] == 254 && buf[6] == 255 {
                    for ch in SelemChannelId::all() {
                        if selem.set_playback_volume(*ch, buf[1] as i64 * max / 100).is_err() {
                            println!("Couldn't change volume")
                        }
                    }
                    println!("{:?}", buf);
                }
            }
        }
    }
}


