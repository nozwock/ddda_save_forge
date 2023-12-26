use std::fs;

use ddda_save::DDDASave;

fn main() {
    let packed_save = fs::read(dbg!(std::env::args().nth(1).unwrap())).unwrap();
    let unpacked_save = DDDASave::try_from(packed_save.as_slice())
        .unwrap()
        .unpack()
        .unwrap();

    fs::write("DDDA.sav.xml", &unpacked_save).unwrap();
    fs::write("DDDA.sav", DDDASave::repack(&unpacked_save).unwrap()).unwrap();
}
