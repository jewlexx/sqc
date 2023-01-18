mod compressor;
mod decompressor;

use std::io::{Read, Write};

pub use compressor::Compressor;
pub use decompressor::Decompressor;

pub fn compress(compress: impl Read, output: impl Write) -> std::io::Result<()> {
    let mut compressor = Compressor::new(compress);

    compressor.compress(output)?;

    Ok(())
}

pub fn decompress(compressed: impl Read, output: impl Write) -> std::io::Result<()> {
    let mut decompressor = Decompressor::new(compressed);

    decompressor.decompress(output);

    Ok(())
}
