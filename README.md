# Installation and setup
Install Rust as described [here](https://www.rust-lang.org/learn/get-started).

To compile and run the program (display usage information), run

```
cargo run --release -- -h
```

To specify different options and arguments, write them after the -- (instead of -h).

# Functions
The program implements an LZ77 compressor and decompressor which can be used for arbitrary byte sequences.
No metadata is written, the parameters (lookback buffer size and maximum length) cannot be changed at runtime
(but can easily changed at compile time).

For little-endian 64 bit integers, the program can optionally perform delta encoding and decoding.
To enable it, use the `-e` flag. In this case, the byte sequence is interpreted as 64 bit integers.
If the input byte size is not divisible by 8, padding with zeros is assumed.

Run `cargo run --release -- -h` to get a usage description.

# Data
In the `data` folder, example data files can be found. The files `integer*` contain 1 million integers in uncompressed,
compressed and delta encoded compressed formats. The integers are from 0 to 10^6 - 1 in order, which is the best
possible case for delta coding. Be advised that delta compressing the integers can be quite slow as finding
matches in the lookback buffer is not optimized.

The file `bscthesis.tex` contains text which can be used to measure the compression without delta coding on real-world data.
