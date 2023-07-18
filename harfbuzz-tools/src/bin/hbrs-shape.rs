use std::env;
use harfbuzz::{Face, Buffer, Font, hb_shape};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {:} <font file> <text>", &args[0]);
    }

    let Ok(face) = Face::new_from_file(&args[1], 0) else {
        eprintln!("Error: could not find font file '{:}'.", &args[1]);
        std::process::exit(1);
    };

    let font = Font::new(&face);
    let mut buffer = Buffer::with(&args[2]);

    buffer.guess_segment_properties();

    let shaped = hb_shape(&font, buffer, &[]);
    let infos = shaped.get_glyph_infos();
    let positions = shaped.get_glyph_positions();

    let chars: Vec<char> = args[2].chars().collect();

    for x in infos.iter().zip(positions.iter()) {
        println!("{} {} {} {} {}", chars[x.0.cluster() as usize], x.0.cluster(), x.0.index(), x.1.x_advance(), x.1.y_advance());
    }

}
