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
    assert_eq!(cpu.pc().wrapping_sub(1), case.r#final.pc, "pc"); // fudge!
    assert_eq!(cpu.sp(), case.r#final.sp, "sp");
    for (k, v) in case.r#final.ram.iter().copied() {
      assert_eq!(ram[k as usize], v, "ram");
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

#[test]
fn test_op_code_0x07() {
  do_json_test("tests_jsmoo/07.json");
}

#[test]
fn test_op_code_0x08() {
  do_json_test("tests_jsmoo/08.json");
}

#[test]
fn test_op_code_0x09() {
  do_json_test("tests_jsmoo/09.json");
}

#[test]
fn test_op_code_0x0a() {
  do_json_test("tests_jsmoo/0a.json");
}

#[test]
fn test_op_code_0x0b() {
  do_json_test("tests_jsmoo/0b.json");
}

#[test]
fn test_op_code_0x0c() {
  do_json_test("tests_jsmoo/0c.json");
}

#[test]
fn test_op_code_0x0d() {
  do_json_test("tests_jsmoo/0d.json");
}

#[test]
fn test_op_code_0x0e() {
  do_json_test("tests_jsmoo/0e.json");
}

#[test]
fn test_op_code_0x0f() {
  do_json_test("tests_jsmoo/0f.json");
}

#[test]
#[cfg(FALSE)]
// TODO: some day we should implement STOP for real.
fn test_op_code_0x10() {
  do_json_test("tests_jsmoo/10.json");
}

#[test]
fn test_op_code_0x11() {
  do_json_test("tests_jsmoo/11.json");
}

#[test]
fn test_op_code_0x12() {
  do_json_test("tests_jsmoo/12.json");
}

#[test]
fn test_op_code_0x13() {
  do_json_test("tests_jsmoo/13.json");
}

#[test]
fn test_op_code_0x14() {
  do_json_test("tests_jsmoo/14.json");
}

#[test]
fn test_op_code_0x15() {
  do_json_test("tests_jsmoo/15.json");
}

#[test]
fn test_op_code_0x16() {
  do_json_test("tests_jsmoo/16.json");
}

#[test]
fn test_op_code_0x17() {
  do_json_test("tests_jsmoo/17.json");
}

#[test]
fn test_op_code_0x18() {
  do_json_test("tests_jsmoo/18.json");
}

#[test]
fn test_op_code_0x19() {
  do_json_test("tests_jsmoo/19.json");
}

#[test]
fn test_op_code_0x1a() {
  do_json_test("tests_jsmoo/1a.json");
}

#[test]
fn test_op_code_0x1b() {
  do_json_test("tests_jsmoo/1b.json");
}

#[test]
fn test_op_code_0x1c() {
  do_json_test("tests_jsmoo/1c.json");
}

#[test]
fn test_op_code_0x1d() {
  do_json_test("tests_jsmoo/1d.json");
}

#[test]
fn test_op_code_0x1e() {
  do_json_test("tests_jsmoo/1e.json");
}

#[test]
fn test_op_code_0x1f() {
  do_json_test("tests_jsmoo/1f.json");
}

#[test]
fn test_op_code_0x20() {
  do_json_test("tests_jsmoo/20.json");
}

#[test]
fn test_op_code_0x21() {
  do_json_test("tests_jsmoo/21.json");
}

#[test]
fn test_op_code_0x22() {
  do_json_test("tests_jsmoo/22.json");
}

#[test]
fn test_op_code_0x23() {
  do_json_test("tests_jsmoo/23.json");
}

#[test]
fn test_op_code_0x24() {
  do_json_test("tests_jsmoo/24.json");
}

#[test]
fn test_op_code_0x25() {
  do_json_test("tests_jsmoo/25.json");
}

#[test]
fn test_op_code_0x26() {
  do_json_test("tests_jsmoo/26.json");
}

#[test]
fn test_op_code_0x27() {
  do_json_test("tests_jsmoo/27.json");
}

#[test]
fn test_op_code_0x28() {
  do_json_test("tests_jsmoo/28.json");
}

#[test]
fn test_op_code_0x29() {
  do_json_test("tests_jsmoo/29.json");
}

#[test]
fn test_op_code_0x2a() {
  do_json_test("tests_jsmoo/2a.json");
}

#[test]
fn test_op_code_0x2b() {
  do_json_test("tests_jsmoo/2b.json");
}

#[test]
fn test_op_code_0x2c() {
  do_json_test("tests_jsmoo/2c.json");
}

#[test]
fn test_op_code_0x2d() {
  do_json_test("tests_jsmoo/2d.json");
}

#[test]
fn test_op_code_0x2e() {
  do_json_test("tests_jsmoo/2e.json");
}

#[test]
fn test_op_code_0x2f() {
  do_json_test("tests_jsmoo/2f.json");
}

#[test]
fn test_op_code_0x30() {
  do_json_test("tests_jsmoo/30.json");
}

#[test]
fn test_op_code_0x31() {
  do_json_test("tests_jsmoo/31.json");
}

#[test]
fn test_op_code_0x32() {
  do_json_test("tests_jsmoo/32.json");
}

#[test]
fn test_op_code_0x33() {
  do_json_test("tests_jsmoo/33.json");
}

#[test]
fn test_op_code_0x34() {
  do_json_test("tests_jsmoo/34.json");
}

#[test]
fn test_op_code_0x35() {
  do_json_test("tests_jsmoo/35.json");
}

#[test]
fn test_op_code_0x36() {
  do_json_test("tests_jsmoo/36.json");
}

#[test]
fn test_op_code_0x37() {
  do_json_test("tests_jsmoo/37.json");
}

#[test]
fn test_op_code_0x38() {
  do_json_test("tests_jsmoo/38.json");
}

#[test]
fn test_op_code_0x39() {
  do_json_test("tests_jsmoo/39.json");
}

#[test]
fn test_op_code_0x3a() {
  do_json_test("tests_jsmoo/3a.json");
}

#[test]
fn test_op_code_0x3b() {
  do_json_test("tests_jsmoo/3b.json");
}

#[test]
fn test_op_code_0x3c() {
  do_json_test("tests_jsmoo/3c.json");
}

#[test]
fn test_op_code_0x3d() {
  do_json_test("tests_jsmoo/3d.json");
}

#[test]
fn test_op_code_0x3e() {
  do_json_test("tests_jsmoo/3e.json");
}

#[test]
fn test_op_code_0x3f() {
  do_json_test("tests_jsmoo/3f.json");
}
