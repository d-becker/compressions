use std::cmp::min;

mod sliding_window;

#[derive(Debug)]
pub struct Block {
    pub distance: usize,
    pub length: usize,
    pub literal: u8,
}

fn longest_common_prefix(left: &[u8], right: &[u8]) -> usize {
    let mut res = 0;
    let mut i = 0;

    while let (Some(b1), Some(b2)) = (left.get(i), right.get(i)) {
        if b1 != b2 {
            break;
        }

        res += 1;
        i += 1;
    }

    res
}

fn longest_match(readback_buffer: &[u8], input: &[u8]) -> (usize, usize) {
    let range = 0..readback_buffer.len();
    let (index, length) = range
        .map(|i| (i, &readback_buffer[i..]))
        .map(|(i, left)| (i, longest_common_prefix(left, input)))
        .max_by_key(|&(_, prefix_len)| prefix_len)
        .unwrap_or((0, 0));

    let distance = if length == 0 {
        0
    } else {
        readback_buffer.len() - index
    };

    (distance, length)
}

fn lz77_encode(bytes: &[u8], distance_bits: usize, length_bits: usize) -> Vec<Block> {
    let readback_size: usize = 2usize.pow(distance_bits as u32) - 1;
    let max_length: usize = 2usize.pow(length_bits as u32) - 1;

    let mut res = Vec::new();

    let mut index = 0;
    while index < bytes.len() {
        let buffer_start = if index <= readback_size {
            0
        } else {
            index - distance_bits
        };

        let buffer_end = min(index, buffer_start + readback_size);

        let (distance, mut length) =
            longest_match(&bytes[buffer_start..buffer_end], &bytes[index..]);
        length = min(length, max_length);

        // If we are at the end of the input, we still need a literal.
        if index + length >= bytes.len() {
            assert!(index + length == bytes.len());
            length -= 1;
        }

        let literal = bytes[index + length];

        res.push(Block {
            distance,
            length,
            literal,
        });

        index += length + 1;
    }

    res
}

fn main() {
    let input = [1, 2, 3, 4, 4, 4, 5, 5, 5, 5, 5, 5];
    let result = lz77_encode(&input, 12, 4);
    println!("Result: {:?}", result);
}
