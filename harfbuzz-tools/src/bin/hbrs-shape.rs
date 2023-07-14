use std::env;
use harfbuzz::{Face, Buffer, Font, hb_shape};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {:} <font file> <text>", &args[0]);
    }

    let Some(face) = Face::new_from_file(&args[1], 0) else {
        eprintln!("Error: could not find font file '{:}'.", &args[1]);
        std::process::exit(1);
    };

    let font = Font::new(&face);
    let buffer = Buffer::with(&args[2]);

    hb_shape(&font, &buffer, &[]);
}
