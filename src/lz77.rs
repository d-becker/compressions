use std::cmp::min;

use crate::bit_packer;

#[derive(Debug, Eq, PartialEq)]
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

pub fn lz77_encode_to_blocks(bytes: &[u8], distance_bits: usize, length_bits: usize) -> Vec<Block> {
    let readback_size: usize = 2usize.pow(distance_bits as u32) - 1;
    let max_length: usize = 2usize.pow(length_bits as u32) - 1;

    let mut res = Vec::new();

    let mut index = 0;
    while index < bytes.len() {
        let buffer_start = if index <= readback_size {
            0
        } else {
            index - readback_size
        };

        let buffer_end = index;

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

#[must_use]
pub fn lz77_decode_blocks(blocks: &[Block]) -> Option<Vec<u8>> {
    let mut res = Vec::new();

    for block in blocks {
        for _ in 0..block.length {
            if block.distance > res.len() {
                return None;
            }

            let index = res.len() - block.distance;
            res.push(res[index]);
        }

        res.push(block.literal);
    }

    Some(res)
}

#[must_use]
pub fn pack_blocks(blocks: &[Block], distance_bits: usize, length_bits: usize) -> Option<Vec<u8>> {
    let mut packer = bit_packer::BitPacker::new();

    for block in blocks {
        packer.pack(block.distance as u64, distance_bits)?;
        packer.pack(block.length as u64, length_bits)?;
        packer.pack(block.literal as u64, 8)?;
    }

    Some(packer.get_bytes())
}

#[must_use]
pub fn unpack_blocks(bytes: &[u8], distance_bits: usize, length_bits: usize) -> Option<Vec<Block>> {
    let block_length = distance_bits + length_bits + 8;
    let no_of_blocks = bytes.len() * 8 / block_length;

    let mut unpacker = bit_packer::BitUnpacker::new(bytes);
    let mut blocks = Vec::new();

    for _ in 0..no_of_blocks {
        let distance = unpacker.unpack(distance_bits)? as usize;
        let length = unpacker.unpack(length_bits)? as usize;
        let literal = unpacker.unpack(8)? as u8;

        blocks.push(Block {
            distance,
            length,
            literal,
        });
    }

    Some(blocks)
}

pub fn lz77_encode(plain: &[u8], distance_bits: usize, length_bits: usize) -> Vec<u8> {
    let blocks = lz77_encode_to_blocks(plain, distance_bits, length_bits);

    // We can safely unwrap because the bit widths are enough.
    pack_blocks(&blocks, distance_bits, length_bits).unwrap()
}

pub fn lz77_decode(encoded: &[u8], distance_bits: usize, length_bits: usize) -> Option<Vec<u8>> {
    let blocks = unpack_blocks(encoded, distance_bits, length_bits)?;
    lz77_decode_blocks(&blocks)
}

#[cfg(test)]
mod tests;
