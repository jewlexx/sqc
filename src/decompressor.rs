use std::io::{BufRead, Write};

pub struct Decompressor<R> {
    compressed: R,
}

// TODO: Ensure memory won't be an issue with larger files

impl<R: BufRead> Decompressor<R> {
    pub fn new(compressed: R) -> Self {
        Self { compressed }
    }

    pub fn decompress(self, mut output: impl Write) -> std::io::Result<()> {
        for line in self.compressed.lines() {
            if let Ok(line) = line {
                // TODO: Better error handling
                let mut parts = line.splitn(2, 'x');

                let byte_raw = parts.next().unwrap();
                let count_raw = parts.next().unwrap();

                let byte: u8 = byte_raw.parse().unwrap();
                let count: usize = count_raw.parse().unwrap();

                let bytes = vec![byte; count];

                output.write_all(&bytes)?;
            } else {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Decompressor;

    #[test]
    fn test_decompress() {
        const COMPRESSED_CHAR: char = 'a';

        let compressed = format!("{}x100\n", COMPRESSED_CHAR as u8);

        let decompressor = Decompressor::new(compressed.as_bytes());

        let mut decompressed = Vec::new();

        decompressor.decompress(&mut decompressed).unwrap();

        assert_eq!(
            COMPRESSED_CHAR.to_string().repeat(100).as_bytes(),
            &decompressed
        );
    }

    #[test]
    fn test_complex() {
        let uncompresssed = "aaxbbb".repeat(10);

        let compressed = format!("{}x2\n{}x1\n{}x3\n", 'a' as u8, 'x' as u8, 'b' as u8).repeat(10);
        let compressed_bytes = compressed.as_bytes();

        let decompressor = Decompressor::new(compressed_bytes);

        let mut decompressed = Vec::new();

        decompressor.decompress(&mut decompressed).unwrap();

        assert_eq!(uncompresssed.as_bytes(), &decompressed)
    }
}
