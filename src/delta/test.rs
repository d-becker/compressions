use super::*;

#[test]
fn to_delta_and_back() {
    let input: Vec<u64> = (0..1000).collect();

    let deltas = to_deltas(&input);

    let back = from_deltas(&deltas);

    assert_eq!(input, back);
}

#[test]
fn to_bytes_and_back() {
    let input: Vec<u64> = (0..1000).collect();

    let bytes = u64s_to_le_bytes(&input);

    let back = le_bytes_to_u64s(&bytes);

    assert_eq!(input, back);
}
