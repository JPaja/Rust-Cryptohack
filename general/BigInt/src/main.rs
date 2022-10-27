fn main() {
    let input = b"11515195063862318899931685488813747395775516287289682636499965282714637259206269";
    let big_int = num_bigint::BigUint::parse_bytes(input, 10).expect("Failed to parse Big integer");
    let bytes = big_int.to_bytes_be();
    let result = ascii::AsciiString::from_ascii(bytes).expect("Failed to parse ascii string");
    println!("{result}");
}
