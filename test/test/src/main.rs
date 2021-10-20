witx_bindgen_rust::import!("../../random.wai.md");

fn main() {
    test_random_bytes();
}

fn test_random_bytes() {
    let mut buffer = [0_u8; 32];
    random::random_bytes(&mut buffer);
}
