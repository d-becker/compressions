use super::*;

use rand::Rng;

#[test]
fn to_blocks_and_back() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let length: usize = rng.gen_range(1000, 10000);
        let mut bytes = Vec::with_capacity(length);

        for _ in 0..length {
            bytes.push(rng.gen());
        }

        let bytes = bytes;

        let encoded = lz77_encode_to_blocks(&bytes, 12, 4);
        let decoded = lz77_decode_blocks(&encoded).unwrap();

        assert_eq!(bytes, decoded);
    }
}

#[test]
fn encode_and_decode_blocks() {
    let mut blocks_builder = Vec::new();
    for i in 0..10 {
        blocks_builder.push(Block {distance: i, length: i, literal: i as u8});
    }

    let blocks = blocks_builder;

    let packed = pack_blocks(&blocks, 12, 4).unwrap();
    assert_eq!(blocks.len() * 3, packed.len());

    let unpacked = unpack_blocks(&packed, 12, 4).unwrap();

    assert_eq!(blocks, unpacked);
}

#[test]
fn encode_and_decode() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let length: usize = rng.gen_range(1000, 10000);
        let mut bytes = Vec::with_capacity(length);

        for _ in 0..length {
            bytes.push(rng.gen());
        }

        let bytes = bytes;

        let encoded = lz77_encode(&bytes, 12, 4);
        let decoded = lz77_decode(&encoded, 12, 4).unwrap();

        assert_eq!(bytes.len(), decoded.len());
        assert_eq!(bytes, decoded);
    }

}
