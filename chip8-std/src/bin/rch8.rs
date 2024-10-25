use std::{fs::File, io::{self, Read}};

use chip8::app::AppState;
use chip8_std::run_rom;

fn main() -> io::Result<()> {
    if let Some(path) = std::env::args().nth(1) {
        let mut f = File::open(path)?;

        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        run_rom(&buffer.as_slice());

    } else {
        println!("no file given");
    }

    Ok(())
}
