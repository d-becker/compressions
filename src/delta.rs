const U64_LEN: usize = std::mem::size_of::<u64>();

pub fn to_deltas(input: &[u64]) -> Vec<u64> {
    if input.is_empty() {
        return Vec::new()
    }

    let mut res = Vec::with_capacity(input.len());

    res.push(input[0]);

    for i in 1..input.len() {
        let prev = input[i - 1];
        let value = input[i];
        let delta = value - prev;
        res.push(delta);
    }

    res
}

pub fn from_deltas(input: &[u64]) -> Vec<u64> {
    if input.is_empty() {
        return Vec::new()
    }

    let mut res = Vec::with_capacity(input.len());

    res.push(input[0]);

    for i in 1..input.len() {
        let prev = res[i - 1];
        let delta = input[i];
        res.push(prev + delta);
    }

    res
}

pub fn le_bytes_to_u64s(bytes: &[u8]) -> Vec<u64> {
    let mut res = Vec::with_capacity((bytes.len() +1) / U64_LEN);

    let full_ints = bytes.len() / U64_LEN;

    for i in 0..full_ints {
        let slice = &bytes[i*U64_LEN..(i+1)*U64_LEN];

        let mut arr: [u8; U64_LEN] = Default::default();
        arr.copy_from_slice(slice);
        let int = u64::from_le_bytes(arr);
        res.push(int);
    }

    let remaining_bytes = bytes.len() % U64_LEN;

    if remaining_bytes > 0 {
        let mut int_bytes: [u8; U64_LEN] = Default::default();
        (&mut int_bytes[0..remaining_bytes]).copy_from_slice(&bytes[full_ints*U64_LEN..]);
        let int = u64::from_le_bytes(int_bytes);
        res.push(int);
    }

    res
}

pub fn u64s_to_le_bytes(ints: &[u64]) -> Vec<u8> {
    let mut res = Vec::with_capacity(ints.len() * U64_LEN);

    for int in ints {
        let arr = int.to_le_bytes();
        res.extend_from_slice(&arr);
    }

    res
}

#[cfg(test)]
mod test;
