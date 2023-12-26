use std::fs;

use ddda_save::DDDASave;

fn main() {
    let packed_save = fs::read(dbg!(std::env::args().nth(1).unwrap())).unwrap();
    fs::write(
        "test.xml",
        DDDASave::try_from(packed_save.as_slice())
            .unwrap()
            .unpack()
            .unwrap(),
    )
    .unwrap();
}
