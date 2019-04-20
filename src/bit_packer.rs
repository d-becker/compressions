use std::cmp::min;

pub struct BitPacker {
    data: Vec<u8>,
    bit_offset: usize,
    buffer: u8
}

impl BitPacker {
    pub fn new() -> BitPacker {
        BitPacker {data: Vec::new(), bit_offset: 0, buffer: 0}
    }

    #[must_use]
    pub fn pack(&mut self, value: u64, bit_width: usize) -> Option<()> {
        let significant_bits = std::mem::size_of::<u64>() * 8 - value.leading_zeros() as usize;
        if significant_bits > bit_width {
            return None;
        }

        let mut remaining_bits = bit_width;
        let mut remaining_value = value;
        while remaining_bits > 0 {
            remaining_value <<= self.bit_offset;
            let v: u8 = remaining_value as u8;
            self.buffer |= v;
            let encoded_bits = min(remaining_bits, 8 - self.bit_offset);

           remaining_bits -= encoded_bits;
            self.bit_offset += encoded_bits;

            if self.bit_offset == 8 {
                self.data.push(self.buffer);
                self.buffer = 0;
                self.bit_offset = 0;
                remaining_value >>= 8;
            }
        }

        Some(())
    }

    pub fn get_bytes(mut self) -> Vec<u8> {
        if self.bit_offset > 0 {
            self.data.push(self.buffer);
        }

        self.data
    }
}

pub struct BitUnpacker<'a> {
    data: &'a [u8],
    byte_offset: usize,
    bit_offset: usize
}

impl<'a> BitUnpacker<'a> {
    pub fn new(data: &[u8]) -> BitUnpacker {
        BitUnpacker {data, byte_offset: 0, bit_offset: 0}
    }

    pub fn unpack(&mut self, bit_width: usize) -> Option<u64> {
        if bit_width > 64 {
            return None;
        }

        let mut res = 0;
        let mut bits_needed = bit_width;

        while bits_needed > 0 {
            let bits_unpacked = min(bits_needed, 8 - self.bit_offset);
            let byte = *self.data.get(self.byte_offset)? as u64;

            // Zero out unneeded high bits.
            let modulus = 1 << (bits_unpacked + self.bit_offset);
            let masked_byte = byte % modulus as u64;

            // Zero out already used low bits.
            let shifted_out_low_bits = masked_byte >> self.bit_offset;

            res |= shifted_out_low_bits << (bit_width - bits_needed);

            bits_needed -= bits_unpacked;
            self.bit_offset += bits_unpacked;
            if self.bit_offset == 8 {
                self.bit_offset = 0;
                self.byte_offset += 1;
            }
        }

        Some(res)
    }
}

#[cfg(test)]
mod tests;
