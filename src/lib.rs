mod compressor;
mod decompressor;

use std::io::{BufRead, Read, Write};

pub use compressor::Compressor;
pub use decompressor::Decompressor;

pub fn compress(compress: impl Read, output: impl Write) -> std::io::Result<()> {
    let mut compressor = Compressor::new(compress);

    compressor.compress(output)?;

    Ok(())
}

pub fn decompress(compressed: impl BufRead, output: impl Write) -> std::io::Result<()> {
    let decompressor = Decompressor::new(compressed);

    decompressor.decompress(output)?;

    Ok(())
}

#[no_mangle]
pub fn compress_bytes(to_compress: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut output = vec![];

    compress(to_compress, &mut output)?;

    Ok(output)
}

#[no_mangle]
pub fn decompress_bytes(compressed: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut output = vec![];

    decompress(compressed, &mut output)?;

    Ok(output)
}
