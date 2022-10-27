fn main() {

    let input = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let bytes = hex::decode(input).expect("Failed to decode hex bytes");
    let result = ascii::AsciiStr::from_ascii(&bytes).expect("Failed to decode ascii string");
    println!("{result}");
}
