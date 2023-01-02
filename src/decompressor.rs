use std::io::{Read, Seek};

pub struct Decompressor<R> {
    // TODO: Maybe don't use string because we might have to represent data larger than one's
    // memory limits
    compressed: R,
}

// Test if mem size of Read.iter() is bearable with large Read

impl<R: Read + Seek> Decompressor<R> {
    pub fn new(compressed: R) -> Self {
        Self { compressed }
    }

    pub fn decompress(&mut self) -> std::io::Result<()> {
        let mut bytes = self.compressed.bytes();

        while let Some(byte) = bytes.next() {}
    }
}

#[cfg(test)]
mod tests {}
