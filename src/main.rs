extern crate alsa;
extern crate serial;

use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

use alsa::mixer::{
    Mixer,
    SelemId,
    SelemChannelId,
};

fn main() {
    let mixer = match Mixer::new("default", false) {
        Ok(mixer) => mixer,
        Err(_) => {
            println!("Couldn't get alsa mixer");
            return
        },
    };
    let selem_id = SelemId::new("Master", 0);
    let selem = match mixer.find_selem(&selem_id) {
        Some(selem) => selem,
        None => {
            println!("Couldn't get selem Master");
            return
        }
    };
    let (_, max) = selem.get_playback_volume_range();

    let mut port = match serial::open("/dev/ttyACM0") {
        Ok(port) => port,
        Err(_) => {
            println!("Could not open serial port");
            return
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
        return
    };
    if port.set_timeout(Duration::from_millis(1000)).is_err() {
        println!("Couldn't set port timeout");
        return
    }

    let mut buf: Vec<u8> = vec![0];
    let mut errors = 0;
    loop {
        if port.read_exact(&mut buf[..]).is_err() {
            println!("Couldn't read from serial port");
            if errors > 10 {
                break
            } else {
                errors += 1;
            }
        } else {
            errors = 0;
            for ch in SelemChannelId::all() {
                if selem.set_playback_volume(*ch, buf[0] as i64 * max / 100).is_err() {
                    println!("Couldn't change volume")
                }
            }
        }
    }
}


