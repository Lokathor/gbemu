#[test]
fn cpu_instrs() {
  let file_bytes = std::fs::read("tests_blargg/cpu_instrs.gb").unwrap();
  assert_eq!(file_bytes[0x147], 0x01);
}
