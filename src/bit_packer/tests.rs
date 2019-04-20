use super::*;

#[test]
fn pack_whole_bytes() {
    let mut packer = BitPacker::new();

    let numbers: [u8; 4] = [1 ,2, 3, 4];

    for number in numbers.iter() {
        let success = packer.pack(*number as u64, 8);
        assert!(success.is_some());
    }

    let result = packer.get_bytes();
    assert_eq!(&numbers, &result[..]);
}

#[test]
fn pack_half_bytes() {
    let mut packer = BitPacker::new();

    let numbers: [u8; 4] = [1 ,2, 3, 4];

    for number in numbers.iter() {
        let success = packer.pack(*number as u64, 4);
        assert!(success.is_some())
    }

    let expected = [33, 67];
    let result = packer.get_bytes();
    assert_eq!(expected, &result[..]);
}

#[test]
fn unpack_whole_bytes() {
    let numbers: [u8; 4] = [1 ,2, 3, 4];
    let mut unpacker = BitUnpacker::new(&numbers);

    for number in numbers.iter() {
        let result = unpacker.unpack(8).unwrap() as u8;
        assert_eq!(*number, result);
    }
}

#[test]
fn unpack_half_bytes() {
    let packed = [33, 67];
    let mut unpacker = BitUnpacker::new(&packed);

    let expected: [u8; 4] = [1 ,2, 3, 4];

    let mut result = Vec::new();
    for _ in expected.iter() {
        result.push(unpacker.unpack(4).unwrap() as u8);
    }

    assert_eq!(expected, &result[..]);
}

#[test]
fn pack_and_unpack() {
    let numbers: Vec<u64> = (500..1000).collect();
    for bit_width in 10..64 {
       let mut packer = BitPacker::new();

       for number in &numbers {
           let success = packer.pack(*number, bit_width);
           assert!(success.is_some());
       }

       let packed = packer.get_bytes();

       let mut unpacker = BitUnpacker::new(&packed);

       for number in &numbers {
           let result = unpacker.unpack(bit_width).unwrap();
           assert_eq!(*number, result);
       }
    }
}
