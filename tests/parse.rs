use gfd::GFD;
use std::fs;

#[test]
fn parse_gsh() {
    let data = fs::read("tests/data/shader.gsh").expect("Failed to read shader.gsh");
    let gfd = GFD::try_from(data.as_ref()).unwrap();

    assert_eq!(
        gfd.blocks.len(),
        5,
        "Expected 5 blocks, got {}",
        gfd.blocks.len()
    );
}
