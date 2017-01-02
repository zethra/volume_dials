extern crate alsa;
extern crate serial;

use std::io;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

use alsa::mixer::{
    Mixer,
    SelemId,
    SelemChannelId,
};

fn main() {
    let mixer = Mixer::new("default", false).unwrap();
    let selem_id = SelemId::new("Master", 0);
    let selem = mixer.find_selem(&selem_id).unwrap();
    let (min, max) = selem.get_playback_volume_range();


    let mut port = serial::open("/dev/ttyACM0").unwrap();
    port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud9600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    });
    port.set_timeout(Duration::from_millis(1000));
    let mut buf: Vec<u8> = vec![0];
    loop {
        port.read(&mut buf[..]);
        for ch in SelemChannelId::all() {
            selem.set_playback_volume(*ch, buf[0] as i64 * max / 100);
        }
    }
}


