use std::io::BufRead;

pub struct Decompressor<R> {
    // TODO: Maybe don't use string because we might have to represent data larger than one's
    // memory limits
    compressed: R,
}

// Test if mem size of Read.iter() is bearable with large Read

impl<R: BufRead> Decompressor<R> {
    pub fn new(compressed: R) -> Self {
        Self { compressed }
    }

    pub fn decompress(mut self) -> std::io::Result<()> {
        loop {
            let mut line = Vec::new();
            self.compressed.read_until(b'\n', &mut line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
