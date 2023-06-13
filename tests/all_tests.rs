use gbemu::SM83;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CpuTestState {
  pc: u16,
  sp: u16,
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  f: u8,
  h: u8,
  l: u8,
  ime: u8,
  ei: Option<u8>,
  ram: Vec<(u16, u8)>,
}

#[derive(Serialize, Deserialize)]
struct TestCase {
  name: String,
  initial: CpuTestState,
  r#final: CpuTestState,
  cycles: Vec<(u16, Option<u8>, String)>,
}

fn do_json_test(file: &str) {
  let str = std::fs::read_to_string(file).unwrap();
  let cases: Vec<TestCase> = serde_json::from_str(&str).unwrap();
  //
  for case in cases {
    println!("Test Case: `{}`", case.name);
    // TODO: figure out what ime/ei is once we're using a test that changes it.
    assert_eq!(case.initial.ime, case.r#final.ime);
    assert_eq!(case.initial.ei, case.r#final.ei);

    // Initialize The State.
    let mut ram = vec![0_u8; 0x1_0000];
    let mut cpu = SM83::default();
    cpu.set_a(case.initial.a);
    cpu.set_b(case.initial.b);
    cpu.set_c(case.initial.c);
    cpu.set_d(case.initial.d);
    cpu.set_e(case.initial.e);
    cpu.set_f(case.initial.f);
    cpu.set_h(case.initial.h);
    cpu.set_l(case.initial.l);
    cpu.set_pc(case.initial.pc);
    cpu.set_sp(case.initial.sp);
    for (k, v) in case.initial.ram.iter().copied() {
      ram[usize::from(k)] = v;
    }
    // move past the boot-wait to the test's instruction and fetch the
    // instruction to test into the queue. We also fudge this later when we
    // check the PC value by subtracting 1 from our PC when comparing to the
    // "expected" value.
    cpu.m_cycle(&mut ram);

    // Run the CPU for the intended number of M-cycles. We don't simulate having
    // a bus with specific pin configurations between cycles, so the exact cycle
    // values aren't important to us.
    for _ in 0..case.cycles.len() {
      cpu.m_cycle(&mut ram);
    }

    // Check The State
    assert_eq!(cpu.a(), case.r#final.a, "a");
    assert_eq!(cpu.b(), case.r#final.b, "b");
    assert_eq!(cpu.c(), case.r#final.c, "c");
    assert_eq!(cpu.d(), case.r#final.d, "d");
    assert_eq!(cpu.e(), case.r#final.e, "e");
    assert_eq!(cpu.f(), case.r#final.f, "f");
    assert_eq!(cpu.h(), case.r#final.h, "h");
    assert_eq!(cpu.l(), case.r#final.l, "l");
    assert_eq!(cpu.pc() - 1, case.r#final.pc, "pc"); // fudge!
    assert_eq!(cpu.sp(), case.r#final.sp, "sp");
    for (k, v) in case.r#final.ram.iter().copied() {
      assert_eq!(ram[k as usize], v);
    }
  }
}

#[test]
fn test_op_code_0x00() {
  do_json_test("tests_jsmoo/00.json");
}

#[test]
fn test_op_code_0x01() {
  do_json_test("tests_jsmoo/01.json");
}

#[test]
fn test_op_code_0x02() {
  do_json_test("tests_jsmoo/02.json");
}

#[test]
fn test_op_code_0x03() {
  do_json_test("tests_jsmoo/03.json");
}

#[test]
fn test_op_code_0x04() {
  do_json_test("tests_jsmoo/04.json");
}

#[test]
fn test_op_code_0x05() {
  do_json_test("tests_jsmoo/05.json");
}

#[test]
fn test_op_code_0x06() {
  do_json_test("tests_jsmoo/06.json");
}
