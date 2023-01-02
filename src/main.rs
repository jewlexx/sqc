mod compressor;

use std::io::Cursor;

use compressor::Compressor;

fn main() {
    const COMPRESSED_CHAR: char = 'a';

    let to_compress = {
        let char_string = COMPRESSED_CHAR.to_string();

        char_string.repeat(100)
    };

    let to_compress_bytes = to_compress.as_bytes().to_vec();

    let mut compressor = Compressor::new(Box::new(Cursor::new(to_compress_bytes)));

    let compressed = compressor.compress().unwrap();

    assert_eq!(compressed, format!("{}x100", COMPRESSED_CHAR as u8));
}
