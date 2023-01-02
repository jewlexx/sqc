use std::{fs::File, io::Read};

pub struct Compressor {
    reader: Box<dyn Read>,
}

impl From<File> for Compressor {
    fn from(file: File) -> Self {
        Self {
            reader: Box::new(file),
        }
    }
}

impl Compressor {
    pub fn new(reader: Box<dyn Read>) -> Self {
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
    pub fn compress(&mut self) -> std::io::Result<String> {
        // The compressed bytes, represented as text
        let mut compressed = String::new();

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
                compressed.push_str(&format!("{}x{cb_count}", buf[0]));
                cb_count = 0;

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

        Ok(compressed)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::compressor::Compressor;

    #[test]
    fn test_compress() {
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
}
