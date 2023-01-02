# Sequential Compression

This is a proof-of-concept compression algorithm that takes sequences of matching bytes and represents them as text.

This allows for unparalleled compression for a string such as "aaaaaaaaaaaaaaaaaaaaaaaaaaa", but falls apart with anything more complex.

For example the string above would be represented as "97x27", 97 being the UTF-8, binary representation of the character "a". This representation takes up a mere 5 bytes whereas the uncompressed string takes up 27 bytes.

Like I said above, this is a proof-of-concept and is not supposed to be used legitimately, especially given it doesn't even support patterns, which is vital for any good compression, bar exceptional circumstances.
