use std::{
    fs::File,
    io::{Read, Write},
};

pub struct Compressor<R> {
    reader: R,
}

impl<R: Read> Compressor<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Read two bytes into a buffer
    /// Create two new variables holding the first byte in the buffer and 0
    /// Check the second byte against the variable above
    /// If they match, keep the above variable the same, and add 1 to the second variable above
    /// Otherwise, if they do not match, set the first variable above to the second byte in the
    /// buffer, and continue to the next iteration of the loop
    /// In the next iteration of the loop, read the next byte from the file, and set the second
    /// byte in the buffer to it
    pub fn compress(&mut self, mut output: impl Write) -> std::io::Result<()> {
        let mut buf = [0u8; 2];
        // This tracks the number of times the current byte, held in position 0 of the buf
        // variable, is repeated and is reset to 0 whenever byte 2 does not match
        let mut cb_count: u128 = 1;
        // This exists purely to temporarily hold the upcoming byte in the buffer
        // This will take up 1 byte in memory for the lifetime of the function, while remaining
        // redundant for most of it :)
        let mut nb = [0u8; 1];

        // Initialize the above buffer with the first two bytes in the reader
        self.reader.read_exact(&mut buf)?;

        // This boolean is set to true when the file is done being read
        // This allows one final iteration to write the final part of the compressed string
        let mut has_finished = false;

        loop {
            if buf[0] == buf[1] && !has_finished {
                cb_count += 1;
            } else {
                writeln!(output, "{}x{cb_count}", buf[0])?;
                cb_count = 1;

                if has_finished {
                    break;
                }
            }

            if self.reader.read_exact(&mut nb).is_ok() {
                buf[0] = buf[1];
                buf[1] = nb[0];
            } else {
                has_finished = true;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read};

    use crate::compressor::Compressor;

    #[test]
    fn test_compress() {
        const COMPRESSED_CHAR: char = 'a';

        let to_compress = {
            let char_string = COMPRESSED_CHAR.to_string();

            char_string.repeat(100)
        };

        let to_compress_bytes = to_compress.as_bytes().to_vec();

        let mut compressor = Compressor::new(Cursor::new(to_compress_bytes));

        let mut compressed = Vec::new();
        compressor.compress(&mut compressed).unwrap();

        assert_eq!(
            compressed,
            format!("{}x100\n", COMPRESSED_CHAR as u8).as_bytes()
        );
    }

    #[test]
    fn test_complex() {
        let compress = "aaxbbb";
        let to_compress = compress.repeat(10);

        let to_compress_bytes = to_compress.as_bytes().to_vec();

        let mut compressor = Compressor::new(Cursor::new(to_compress_bytes));

        let mut compressed = Vec::new();
        compressor.compress(&mut compressed).unwrap();

        assert_eq!(
            compressed,
            format!("{}x2\n{}x1\n{}x3\n", 'a' as u8, 'x' as u8, 'b' as u8)
                .repeat(10)
                .as_bytes()
        );
    }
}
