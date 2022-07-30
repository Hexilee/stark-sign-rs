use num_bigint::BigInt;

fn main() {
    let (s, mut data) = BigInt::parse_bytes(
        b"126159b7ef616fd9bef9023063ae51daf6037bc512e35dc9e4d4dcdc9bcd3e4",
        16,
    )
    .unwrap()
    .to_bytes_be();
    println!("{:?}", data);
    data = vec![
        18, 97, 89, 183, 239, 97, 111, 217, 190, 249, 2, 48, 99, 174, 81, 218, 246, 3, 123, 197,
        18, 227, 93, 201, 228, 212, 220, 220, 155, 205, 62, 64,
    ];
    let l = BigInt::from_bytes_be(s, &data);
    println!("{l:x}");
}
