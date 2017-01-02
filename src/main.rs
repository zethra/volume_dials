extern crate alsa;

use alsa::mixer::{
    Mixer,
    SelemId,
    SelemChannelId,
};

fn main() {
    let volume = 70;
    let mixer = Mixer::new("default", false).unwrap();
    let selem_id = SelemId::new("Master", 0);
    let selem = mixer.find_selem(&selem_id).unwrap();
    let (min, max) = selem.get_playback_volume_range();
    println!("{}, {}", min, max);
    for ch in SelemChannelId::all() {
        selem.set_playback_volume(*ch, volume * max / 100);
    }
}


