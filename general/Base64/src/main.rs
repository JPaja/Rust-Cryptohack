fn main() {
    let input = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
    let hex_bytes = hex::decode(input).expect("Failed to decode hex string");
    let result = base64::encode(hex_bytes);
    print!("{result}");
}
