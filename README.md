# Sequential Compression

This is a proof-of-concept, "toy", compression algorithm that takes sequences of matching bytes and represents them as text.

This allows for unparalleled compression for a string such as `aaaaaaaaaaaaaaaaaaaaaaaaaaa`, but falls apart with anything more complex.

For example the string above would be represented as `97x27`, 97 being the UTF-8, binary representation of the character "a". This representation takes up a mere 5 bytes whereas the uncompressed string takes up 27 bytes.

## Benchmarks

I compressed a 1GB file was `0` (not the character a binary 0) repeated 8 billion times. This was compressed to the string `0x8000000000`, which is equivalent to about a `9999999985%` decrease in size.

By comparison if you try to compress the `Cargo.toml` file in this repository, it goes from `180 bytes` to `977 bytes` which is a `442.778%` increase in size.

## Spec

Every compression algorithm needs a specification :)

- Compressed files must be represented as plain text files using UTF-8 encoding.
- Every sequence of bytes is represented as the byte, represented as an integer, followed by the character `x`, followed by the number of times it was repeated.
- Every sequence is newline delimited

**Made with 💗 by Juliette Cordor**
