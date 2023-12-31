#![allow(bad_style)]

use gbemu::cpu::SM83;
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
    cpu.set_ime(case.initial.ime != 0);
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
    assert_eq!(cpu.ime(), case.r#final.ime != 0, "ime");
    for (k, v) in case.r#final.ram.iter().copied() {
      assert_eq!(ram[k as usize], v, "ram");
    }
  }
}

#[test]
fn op_code_0x00() {
  do_json_test("tests/jsmoo/00.json");
}
#[test]
fn op_code_0x01() {
  do_json_test("tests/jsmoo/01.json");
}
#[test]
fn op_code_0x02() {
  do_json_test("tests/jsmoo/02.json");
}
#[test]
fn op_code_0x03() {
  do_json_test("tests/jsmoo/03.json");
}
#[test]
fn op_code_0x04() {
  do_json_test("tests/jsmoo/04.json");
}
#[test]
fn op_code_0x05() {
  do_json_test("tests/jsmoo/05.json");
}
#[test]
fn op_code_0x06() {
  do_json_test("tests/jsmoo/06.json");
}
#[test]
fn op_code_0x07() {
  do_json_test("tests/jsmoo/07.json");
}
#[test]
fn op_code_0x08() {
  do_json_test("tests/jsmoo/08.json");
}
#[test]
fn op_code_0x09() {
  do_json_test("tests/jsmoo/09.json");
}
#[test]
fn op_code_0x0A() {
  do_json_test("tests/jsmoo/0a.json");
}
#[test]
fn op_code_0x0B() {
  do_json_test("tests/jsmoo/0b.json");
}
#[test]
fn op_code_0x0C() {
  do_json_test("tests/jsmoo/0c.json");
}
#[test]
fn op_code_0x0D() {
  do_json_test("tests/jsmoo/0d.json");
}
#[test]
fn op_code_0x0E() {
  do_json_test("tests/jsmoo/0e.json");
}
#[test]
fn op_code_0x0F() {
  do_json_test("tests/jsmoo/0f.json");
}
#[test]
#[cfg(FALSE)]
fn op_code_0x10() {
  // STOP
  do_json_test("tests/jsmoo/10.json");
}
#[test]
fn op_code_0x11() {
  do_json_test("tests/jsmoo/11.json");
}
#[test]
fn op_code_0x12() {
  do_json_test("tests/jsmoo/12.json");
}
#[test]
fn op_code_0x13() {
  do_json_test("tests/jsmoo/13.json");
}
#[test]
fn op_code_0x14() {
  do_json_test("tests/jsmoo/14.json");
}
#[test]
fn op_code_0x15() {
  do_json_test("tests/jsmoo/15.json");
}
#[test]
fn op_code_0x16() {
  do_json_test("tests/jsmoo/16.json");
}
#[test]
fn op_code_0x17() {
  do_json_test("tests/jsmoo/17.json");
}
#[test]
fn op_code_0x18() {
  do_json_test("tests/jsmoo/18.json");
}
#[test]
fn op_code_0x19() {
  do_json_test("tests/jsmoo/19.json");
}
#[test]
fn op_code_0x1A() {
  do_json_test("tests/jsmoo/1a.json");
}
#[test]
fn op_code_0x1B() {
  do_json_test("tests/jsmoo/1b.json");
}
#[test]
fn op_code_0x1C() {
  do_json_test("tests/jsmoo/1c.json");
}
#[test]
fn op_code_0x1D() {
  do_json_test("tests/jsmoo/1d.json");
}
#[test]
fn op_code_0x1E() {
  do_json_test("tests/jsmoo/1e.json");
}
#[test]
fn op_code_0x1F() {
  do_json_test("tests/jsmoo/1f.json");
}
#[test]
fn op_code_0x20() {
  do_json_test("tests/jsmoo/20.json");
}
#[test]
fn op_code_0x21() {
  do_json_test("tests/jsmoo/21.json");
}
#[test]
fn op_code_0x22() {
  do_json_test("tests/jsmoo/22.json");
}
#[test]
fn op_code_0x23() {
  do_json_test("tests/jsmoo/23.json");
}
#[test]
fn op_code_0x24() {
  do_json_test("tests/jsmoo/24.json");
}
#[test]
fn op_code_0x25() {
  do_json_test("tests/jsmoo/25.json");
}
#[test]
fn op_code_0x26() {
  do_json_test("tests/jsmoo/26.json");
}
#[test]
fn op_code_0x27() {
  do_json_test("tests/jsmoo/27.json");
}
#[test]
fn op_code_0x28() {
  do_json_test("tests/jsmoo/28.json");
}
#[test]
fn op_code_0x29() {
  do_json_test("tests/jsmoo/29.json");
}
#[test]
fn op_code_0x2A() {
  do_json_test("tests/jsmoo/2a.json");
}
#[test]
fn op_code_0x2B() {
  do_json_test("tests/jsmoo/2b.json");
}
#[test]
fn op_code_0x2C() {
  do_json_test("tests/jsmoo/2c.json");
}
#[test]
fn op_code_0x2D() {
  do_json_test("tests/jsmoo/2d.json");
}
#[test]
fn op_code_0x2E() {
  do_json_test("tests/jsmoo/2e.json");
}
#[test]
fn op_code_0x2F() {
  do_json_test("tests/jsmoo/2f.json");
}
#[test]
fn op_code_0x30() {
  do_json_test("tests/jsmoo/30.json");
}
#[test]
fn op_code_0x31() {
  do_json_test("tests/jsmoo/31.json");
}
#[test]
fn op_code_0x32() {
  do_json_test("tests/jsmoo/32.json");
}
#[test]
fn op_code_0x33() {
  do_json_test("tests/jsmoo/33.json");
}
#[test]
fn op_code_0x34() {
  do_json_test("tests/jsmoo/34.json");
}
#[test]
fn op_code_0x35() {
  do_json_test("tests/jsmoo/35.json");
}
#[test]
fn op_code_0x36() {
  do_json_test("tests/jsmoo/36.json");
}
#[test]
fn op_code_0x37() {
  do_json_test("tests/jsmoo/37.json");
}
#[test]
fn op_code_0x38() {
  do_json_test("tests/jsmoo/38.json");
}
#[test]
fn op_code_0x39() {
  do_json_test("tests/jsmoo/39.json");
}
#[test]
fn op_code_0x3A() {
  do_json_test("tests/jsmoo/3a.json");
}
#[test]
fn op_code_0x3B() {
  do_json_test("tests/jsmoo/3b.json");
}
#[test]
fn op_code_0x3C() {
  do_json_test("tests/jsmoo/3c.json");
}
#[test]
fn op_code_0x3D() {
  do_json_test("tests/jsmoo/3d.json");
}
#[test]
fn op_code_0x3E() {
  do_json_test("tests/jsmoo/3e.json");
}
#[test]
fn op_code_0x3F() {
  do_json_test("tests/jsmoo/3f.json");
}
#[test]
fn op_code_0x40() {
  do_json_test("tests/jsmoo/40.json");
}
#[test]
fn op_code_0x41() {
  do_json_test("tests/jsmoo/41.json");
}
#[test]
fn op_code_0x42() {
  do_json_test("tests/jsmoo/42.json");
}
#[test]
fn op_code_0x43() {
  do_json_test("tests/jsmoo/43.json");
}
#[test]
fn op_code_0x44() {
  do_json_test("tests/jsmoo/44.json");
}
#[test]
fn op_code_0x45() {
  do_json_test("tests/jsmoo/45.json");
}
#[test]
fn op_code_0x46() {
  do_json_test("tests/jsmoo/46.json");
}
#[test]
fn op_code_0x47() {
  do_json_test("tests/jsmoo/47.json");
}
#[test]
fn op_code_0x48() {
  do_json_test("tests/jsmoo/48.json");
}
#[test]
fn op_code_0x49() {
  do_json_test("tests/jsmoo/49.json");
}
#[test]
fn op_code_0x4A() {
  do_json_test("tests/jsmoo/4a.json");
}
#[test]
fn op_code_0x4B() {
  do_json_test("tests/jsmoo/4b.json");
}
#[test]
fn op_code_0x4C() {
  do_json_test("tests/jsmoo/4c.json");
}
#[test]
fn op_code_0x4D() {
  do_json_test("tests/jsmoo/4d.json");
}
#[test]
fn op_code_0x4E() {
  do_json_test("tests/jsmoo/4e.json");
}
#[test]
fn op_code_0x4F() {
  do_json_test("tests/jsmoo/4f.json");
}
#[test]
fn op_code_0x50() {
  do_json_test("tests/jsmoo/50.json");
}
#[test]
fn op_code_0x51() {
  do_json_test("tests/jsmoo/51.json");
}
#[test]
fn op_code_0x52() {
  do_json_test("tests/jsmoo/52.json");
}
#[test]
fn op_code_0x53() {
  do_json_test("tests/jsmoo/53.json");
}
#[test]
fn op_code_0x54() {
  do_json_test("tests/jsmoo/54.json");
}
#[test]
fn op_code_0x55() {
  do_json_test("tests/jsmoo/55.json");
}
#[test]
fn op_code_0x56() {
  do_json_test("tests/jsmoo/56.json");
}
#[test]
fn op_code_0x57() {
  do_json_test("tests/jsmoo/57.json");
}
#[test]
fn op_code_0x58() {
  do_json_test("tests/jsmoo/58.json");
}
#[test]
fn op_code_0x59() {
  do_json_test("tests/jsmoo/59.json");
}
#[test]
fn op_code_0x5A() {
  do_json_test("tests/jsmoo/5a.json");
}
#[test]
fn op_code_0x5B() {
  do_json_test("tests/jsmoo/5b.json");
}
#[test]
fn op_code_0x5C() {
  do_json_test("tests/jsmoo/5c.json");
}
#[test]
fn op_code_0x5D() {
  do_json_test("tests/jsmoo/5d.json");
}
#[test]
fn op_code_0x5E() {
  do_json_test("tests/jsmoo/5e.json");
}
#[test]
fn op_code_0x5F() {
  do_json_test("tests/jsmoo/5f.json");
}
#[test]
fn op_code_0x60() {
  do_json_test("tests/jsmoo/60.json");
}
#[test]
fn op_code_0x61() {
  do_json_test("tests/jsmoo/61.json");
}
#[test]
fn op_code_0x62() {
  do_json_test("tests/jsmoo/62.json");
}
#[test]
fn op_code_0x63() {
  do_json_test("tests/jsmoo/63.json");
}
#[test]
fn op_code_0x64() {
  do_json_test("tests/jsmoo/64.json");
}
#[test]
fn op_code_0x65() {
  do_json_test("tests/jsmoo/65.json");
}
#[test]
fn op_code_0x66() {
  do_json_test("tests/jsmoo/66.json");
}
#[test]
fn op_code_0x67() {
  do_json_test("tests/jsmoo/67.json");
}
#[test]
fn op_code_0x68() {
  do_json_test("tests/jsmoo/68.json");
}
#[test]
fn op_code_0x69() {
  do_json_test("tests/jsmoo/69.json");
}
#[test]
fn op_code_0x6A() {
  do_json_test("tests/jsmoo/6a.json");
}
#[test]
fn op_code_0x6B() {
  do_json_test("tests/jsmoo/6b.json");
}
#[test]
fn op_code_0x6C() {
  do_json_test("tests/jsmoo/6c.json");
}
#[test]
fn op_code_0x6D() {
  do_json_test("tests/jsmoo/6d.json");
}
#[test]
fn op_code_0x6E() {
  do_json_test("tests/jsmoo/6e.json");
}
#[test]
fn op_code_0x6F() {
  do_json_test("tests/jsmoo/6f.json");
}
#[test]
fn op_code_0x70() {
  do_json_test("tests/jsmoo/70.json");
}
#[test]
fn op_code_0x71() {
  do_json_test("tests/jsmoo/71.json");
}
#[test]
fn op_code_0x72() {
  do_json_test("tests/jsmoo/72.json");
}
#[test]
fn op_code_0x73() {
  do_json_test("tests/jsmoo/73.json");
}
#[test]
fn op_code_0x74() {
  do_json_test("tests/jsmoo/74.json");
}
#[test]
fn op_code_0x75() {
  do_json_test("tests/jsmoo/75.json");
}
#[test]
#[cfg(FALSE)]
fn op_code_0x76() {
  // HALT
  do_json_test("tests/jsmoo/76.json");
}
#[test]
fn op_code_0x77() {
  do_json_test("tests/jsmoo/77.json");
}
#[test]
fn op_code_0x78() {
  do_json_test("tests/jsmoo/78.json");
}
#[test]
fn op_code_0x79() {
  do_json_test("tests/jsmoo/79.json");
}
#[test]
fn op_code_0x7A() {
  do_json_test("tests/jsmoo/7a.json");
}
#[test]
fn op_code_0x7B() {
  do_json_test("tests/jsmoo/7b.json");
}
#[test]
fn op_code_0x7C() {
  do_json_test("tests/jsmoo/7c.json");
}
#[test]
fn op_code_0x7D() {
  do_json_test("tests/jsmoo/7d.json");
}
#[test]
fn op_code_0x7E() {
  do_json_test("tests/jsmoo/7e.json");
}
#[test]
fn op_code_0x7F() {
  do_json_test("tests/jsmoo/7f.json");
}
#[test]
fn op_code_0x80() {
  do_json_test("tests/jsmoo/80.json");
}
#[test]
fn op_code_0x81() {
  do_json_test("tests/jsmoo/81.json");
}
#[test]
fn op_code_0x82() {
  do_json_test("tests/jsmoo/82.json");
}
#[test]
fn op_code_0x83() {
  do_json_test("tests/jsmoo/83.json");
}
#[test]
fn op_code_0x84() {
  do_json_test("tests/jsmoo/84.json");
}
#[test]
fn op_code_0x85() {
  do_json_test("tests/jsmoo/85.json");
}
#[test]
fn op_code_0x86() {
  do_json_test("tests/jsmoo/86.json");
}
#[test]
fn op_code_0x87() {
  do_json_test("tests/jsmoo/87.json");
}
#[test]
fn op_code_0x88() {
  do_json_test("tests/jsmoo/88.json");
}
#[test]
fn op_code_0x89() {
  do_json_test("tests/jsmoo/89.json");
}
#[test]
fn op_code_0x8A() {
  do_json_test("tests/jsmoo/8a.json");
}
#[test]
fn op_code_0x8B() {
  do_json_test("tests/jsmoo/8b.json");
}
#[test]
fn op_code_0x8C() {
  do_json_test("tests/jsmoo/8c.json");
}
#[test]
fn op_code_0x8D() {
  do_json_test("tests/jsmoo/8d.json");
}
#[test]
fn op_code_0x8E() {
  do_json_test("tests/jsmoo/8e.json");
}
#[test]
fn op_code_0x8F() {
  do_json_test("tests/jsmoo/8f.json");
}
#[test]
fn op_code_0x90() {
  do_json_test("tests/jsmoo/90.json");
}
#[test]
fn op_code_0x91() {
  do_json_test("tests/jsmoo/91.json");
}
#[test]
fn op_code_0x92() {
  do_json_test("tests/jsmoo/92.json");
}
#[test]
fn op_code_0x93() {
  do_json_test("tests/jsmoo/93.json");
}
#[test]
fn op_code_0x94() {
  do_json_test("tests/jsmoo/94.json");
}
#[test]
fn op_code_0x95() {
  do_json_test("tests/jsmoo/95.json");
}
#[test]
fn op_code_0x96() {
  do_json_test("tests/jsmoo/96.json");
}
#[test]
fn op_code_0x97() {
  do_json_test("tests/jsmoo/97.json");
}
#[test]
fn op_code_0x98() {
  do_json_test("tests/jsmoo/98.json");
}
#[test]
fn op_code_0x99() {
  do_json_test("tests/jsmoo/99.json");
}
#[test]
fn op_code_0x9A() {
  do_json_test("tests/jsmoo/9a.json");
}
#[test]
fn op_code_0x9B() {
  do_json_test("tests/jsmoo/9b.json");
}
#[test]
fn op_code_0x9C() {
  do_json_test("tests/jsmoo/9c.json");
}
#[test]
fn op_code_0x9D() {
  do_json_test("tests/jsmoo/9d.json");
}
#[test]
fn op_code_0x9E() {
  do_json_test("tests/jsmoo/9e.json");
}
#[test]
fn op_code_0x9F() {
  do_json_test("tests/jsmoo/9f.json");
}
#[test]
fn op_code_0xA0() {
  do_json_test("tests/jsmoo/a0.json");
}
#[test]
fn op_code_0xA1() {
  do_json_test("tests/jsmoo/a1.json");
}
#[test]
fn op_code_0xA2() {
  do_json_test("tests/jsmoo/a2.json");
}
#[test]
fn op_code_0xA3() {
  do_json_test("tests/jsmoo/a3.json");
}
#[test]
fn op_code_0xA4() {
  do_json_test("tests/jsmoo/a4.json");
}
#[test]
fn op_code_0xA5() {
  do_json_test("tests/jsmoo/a5.json");
}
#[test]
fn op_code_0xA6() {
  do_json_test("tests/jsmoo/a6.json");
}
#[test]
fn op_code_0xA7() {
  do_json_test("tests/jsmoo/a7.json");
}
#[test]
fn op_code_0xA8() {
  do_json_test("tests/jsmoo/a8.json");
}
#[test]
fn op_code_0xA9() {
  do_json_test("tests/jsmoo/a9.json");
}
#[test]
fn op_code_0xAA() {
  do_json_test("tests/jsmoo/aa.json");
}
#[test]
fn op_code_0xAB() {
  do_json_test("tests/jsmoo/ab.json");
}
#[test]
fn op_code_0xAC() {
  do_json_test("tests/jsmoo/ac.json");
}
#[test]
fn op_code_0xAD() {
  do_json_test("tests/jsmoo/ad.json");
}
#[test]
fn op_code_0xAE() {
  do_json_test("tests/jsmoo/ae.json");
}
#[test]
fn op_code_0xAF() {
  do_json_test("tests/jsmoo/af.json");
}
#[test]
fn op_code_0xB0() {
  do_json_test("tests/jsmoo/b0.json");
}
#[test]
fn op_code_0xB1() {
  do_json_test("tests/jsmoo/b1.json");
}
#[test]
fn op_code_0xB2() {
  do_json_test("tests/jsmoo/b2.json");
}
#[test]
fn op_code_0xB3() {
  do_json_test("tests/jsmoo/b3.json");
}
#[test]
fn op_code_0xB4() {
  do_json_test("tests/jsmoo/b4.json");
}
#[test]
fn op_code_0xB5() {
  do_json_test("tests/jsmoo/b5.json");
}
#[test]
fn op_code_0xB6() {
  do_json_test("tests/jsmoo/b6.json");
}
#[test]
fn op_code_0xB7() {
  do_json_test("tests/jsmoo/b7.json");
}
#[test]
fn op_code_0xB8() {
  do_json_test("tests/jsmoo/b8.json");
}
#[test]
fn op_code_0xB9() {
  do_json_test("tests/jsmoo/b9.json");
}
#[test]
fn op_code_0xBA() {
  do_json_test("tests/jsmoo/ba.json");
}
#[test]
fn op_code_0xBB() {
  do_json_test("tests/jsmoo/bb.json");
}
#[test]
fn op_code_0xBC() {
  do_json_test("tests/jsmoo/bc.json");
}
#[test]
fn op_code_0xBD() {
  do_json_test("tests/jsmoo/bd.json");
}
#[test]
fn op_code_0xBE() {
  do_json_test("tests/jsmoo/be.json");
}
#[test]
fn op_code_0xBF() {
  do_json_test("tests/jsmoo/bf.json");
}
#[test]
fn op_code_0xC0() {
  do_json_test("tests/jsmoo/c0.json");
}
#[test]
fn op_code_0xC1() {
  do_json_test("tests/jsmoo/c1.json");
}
#[test]
fn op_code_0xC2() {
  do_json_test("tests/jsmoo/c2.json");
}
#[test]
fn op_code_0xC3() {
  do_json_test("tests/jsmoo/c3.json");
}
#[test]
fn op_code_0xC4() {
  do_json_test("tests/jsmoo/c4.json");
}
#[test]
fn op_code_0xC5() {
  do_json_test("tests/jsmoo/c5.json");
}
#[test]
fn op_code_0xC6() {
  do_json_test("tests/jsmoo/c6.json");
}
#[test]
fn op_code_0xC7() {
  do_json_test("tests/jsmoo/c7.json");
}
#[test]
fn op_code_0xC8() {
  do_json_test("tests/jsmoo/c8.json");
}
#[test]
fn op_code_0xC9() {
  do_json_test("tests/jsmoo/c9.json");
}
#[test]
fn op_code_0xCA() {
  do_json_test("tests/jsmoo/ca.json");
}
#[test]
fn op_code_0xCC() {
  do_json_test("tests/jsmoo/cc.json");
}
#[test]
fn op_code_0xCD() {
  do_json_test("tests/jsmoo/cd.json");
}
#[test]
fn op_code_0xCE() {
  do_json_test("tests/jsmoo/ce.json");
}
#[test]
fn op_code_0xCF() {
  do_json_test("tests/jsmoo/cf.json");
}
#[test]
fn op_code_0xD0() {
  do_json_test("tests/jsmoo/d0.json");
}
#[test]
fn op_code_0xD1() {
  do_json_test("tests/jsmoo/d1.json");
}
#[test]
fn op_code_0xD2() {
  do_json_test("tests/jsmoo/d2.json");
}
#[test]
fn op_code_0xD4() {
  do_json_test("tests/jsmoo/d4.json");
}
#[test]
fn op_code_0xD5() {
  do_json_test("tests/jsmoo/d5.json");
}
#[test]
fn op_code_0xD6() {
  do_json_test("tests/jsmoo/d6.json");
}
#[test]
fn op_code_0xD7() {
  do_json_test("tests/jsmoo/d7.json");
}
#[test]
fn op_code_0xD8() {
  do_json_test("tests/jsmoo/d8.json");
}
#[test]
fn op_code_0xD9() {
  do_json_test("tests/jsmoo/d9.json");
}
#[test]
fn op_code_0xDA() {
  do_json_test("tests/jsmoo/da.json");
}
#[test]
fn op_code_0xDC() {
  do_json_test("tests/jsmoo/dc.json");
}
#[test]
fn op_code_0xDE() {
  do_json_test("tests/jsmoo/de.json");
}
#[test]
fn op_code_0xDF() {
  do_json_test("tests/jsmoo/df.json");
}
#[test]
fn op_code_0xE0() {
  do_json_test("tests/jsmoo/e0.json");
}
#[test]
fn op_code_0xE1() {
  do_json_test("tests/jsmoo/e1.json");
}
#[test]
fn op_code_0xE2() {
  do_json_test("tests/jsmoo/e2.json");
}
#[test]
fn op_code_0xE5() {
  do_json_test("tests/jsmoo/e5.json");
}
#[test]
fn op_code_0xE6() {
  do_json_test("tests/jsmoo/e6.json");
}
#[test]
fn op_code_0xE7() {
  do_json_test("tests/jsmoo/e7.json");
}
#[test]
fn op_code_0xE8() {
  do_json_test("tests/jsmoo/e8.json");
}
#[test]
fn op_code_0xE9() {
  do_json_test("tests/jsmoo/e9.json");
}
#[test]
fn op_code_0xEA() {
  do_json_test("tests/jsmoo/ea.json");
}
#[test]
fn op_code_0xEE() {
  do_json_test("tests/jsmoo/ee.json");
}
#[test]
fn op_code_0xEF() {
  do_json_test("tests/jsmoo/ef.json");
}
#[test]
fn op_code_0xF0() {
  do_json_test("tests/jsmoo/f0.json");
}
#[test]
fn op_code_0xF1() {
  do_json_test("tests/jsmoo/f1.json");
}
#[test]
fn op_code_0xF2() {
  do_json_test("tests/jsmoo/f2.json");
}
#[test]
fn op_code_0xF3() {
  do_json_test("tests/jsmoo/f3.json");
}
#[test]
fn op_code_0xF5() {
  do_json_test("tests/jsmoo/f5.json");
}
#[test]
fn op_code_0xF6() {
  do_json_test("tests/jsmoo/f6.json");
}
#[test]
fn op_code_0xF7() {
  do_json_test("tests/jsmoo/f7.json");
}
#[test]
fn op_code_0xF8() {
  do_json_test("tests/jsmoo/f8.json");
}
#[test]
fn op_code_0xF9() {
  do_json_test("tests/jsmoo/f9.json");
}
#[test]
fn op_code_0xFA() {
  do_json_test("tests/jsmoo/fa.json");
}
#[test]
fn op_code_0xFB() {
  do_json_test("tests/jsmoo/fb.json");
}
#[test]
fn op_code_0xFE() {
  do_json_test("tests/jsmoo/fe.json");
}
#[test]
fn op_code_0xFF() {
  do_json_test("tests/jsmoo/ff.json");
}

#[test]
fn op_code_0xCB_0x00() {
  do_json_test("tests/jsmoo/cb 00.json");
}
#[test]
fn op_code_0xCB_0x01() {
  do_json_test("tests/jsmoo/cb 01.json");
}
#[test]
fn op_code_0xCB_0x02() {
  do_json_test("tests/jsmoo/cb 02.json");
}
#[test]
fn op_code_0xCB_0x03() {
  do_json_test("tests/jsmoo/cb 03.json");
}
#[test]
fn op_code_0xCB_0x04() {
  do_json_test("tests/jsmoo/cb 04.json");
}
#[test]
fn op_code_0xCB_0x05() {
  do_json_test("tests/jsmoo/cb 05.json");
}
#[test]
fn op_code_0xCB_0x06() {
  do_json_test("tests/jsmoo/cb 06.json");
}
#[test]
fn op_code_0xCB_0x07() {
  do_json_test("tests/jsmoo/cb 07.json");
}
#[test]
fn op_code_0xCB_0x08() {
  do_json_test("tests/jsmoo/cb 08.json");
}
#[test]
fn op_code_0xCB_0x09() {
  do_json_test("tests/jsmoo/cb 09.json");
}
#[test]
fn op_code_0xCB_0x0A() {
  do_json_test("tests/jsmoo/cb 0a.json");
}
#[test]
fn op_code_0xCB_0x0B() {
  do_json_test("tests/jsmoo/cb 0b.json");
}
#[test]
fn op_code_0xCB_0x0C() {
  do_json_test("tests/jsmoo/cb 0c.json");
}
#[test]
fn op_code_0xCB_0x0D() {
  do_json_test("tests/jsmoo/cb 0d.json");
}
#[test]
fn op_code_0xCB_0x0E() {
  do_json_test("tests/jsmoo/cb 0e.json");
}
#[test]
fn op_code_0xCB_0x0F() {
  do_json_test("tests/jsmoo/cb 0f.json");
}
#[test]
fn op_code_0xCB_0x10() {
  do_json_test("tests/jsmoo/cb 10.json");
}
#[test]
fn op_code_0xCB_0x11() {
  do_json_test("tests/jsmoo/cb 11.json");
}
#[test]
fn op_code_0xCB_0x12() {
  do_json_test("tests/jsmoo/cb 12.json");
}
#[test]
fn op_code_0xCB_0x13() {
  do_json_test("tests/jsmoo/cb 13.json");
}
#[test]
fn op_code_0xCB_0x14() {
  do_json_test("tests/jsmoo/cb 14.json");
}
#[test]
fn op_code_0xCB_0x15() {
  do_json_test("tests/jsmoo/cb 15.json");
}
#[test]
fn op_code_0xCB_0x16() {
  do_json_test("tests/jsmoo/cb 16.json");
}
#[test]
fn op_code_0xCB_0x17() {
  do_json_test("tests/jsmoo/cb 17.json");
}
#[test]
fn op_code_0xCB_0x18() {
  do_json_test("tests/jsmoo/cb 18.json");
}
#[test]
fn op_code_0xCB_0x19() {
  do_json_test("tests/jsmoo/cb 19.json");
}
#[test]
fn op_code_0xCB_0x1A() {
  do_json_test("tests/jsmoo/cb 1a.json");
}
#[test]
fn op_code_0xCB_0x1B() {
  do_json_test("tests/jsmoo/cb 1b.json");
}
#[test]
fn op_code_0xCB_0x1C() {
  do_json_test("tests/jsmoo/cb 1c.json");
}
#[test]
fn op_code_0xCB_0x1D() {
  do_json_test("tests/jsmoo/cb 1d.json");
}
#[test]
fn op_code_0xCB_0x1E() {
  do_json_test("tests/jsmoo/cb 1e.json");
}
#[test]
fn op_code_0xCB_0x1F() {
  do_json_test("tests/jsmoo/cb 1f.json");
}
#[test]
fn op_code_0xCB_0x20() {
  do_json_test("tests/jsmoo/cb 20.json");
}
#[test]
fn op_code_0xCB_0x21() {
  do_json_test("tests/jsmoo/cb 21.json");
}
#[test]
fn op_code_0xCB_0x22() {
  do_json_test("tests/jsmoo/cb 22.json");
}
#[test]
fn op_code_0xCB_0x23() {
  do_json_test("tests/jsmoo/cb 23.json");
}
#[test]
fn op_code_0xCB_0x24() {
  do_json_test("tests/jsmoo/cb 24.json");
}
#[test]
fn op_code_0xCB_0x25() {
  do_json_test("tests/jsmoo/cb 25.json");
}
#[test]
fn op_code_0xCB_0x26() {
  do_json_test("tests/jsmoo/cb 26.json");
}
#[test]
fn op_code_0xCB_0x27() {
  do_json_test("tests/jsmoo/cb 27.json");
}
#[test]
fn op_code_0xCB_0x28() {
  do_json_test("tests/jsmoo/cb 28.json");
}
#[test]
fn op_code_0xCB_0x29() {
  do_json_test("tests/jsmoo/cb 29.json");
}
#[test]
fn op_code_0xCB_0x2A() {
  do_json_test("tests/jsmoo/cb 2a.json");
}
#[test]
fn op_code_0xCB_0x2B() {
  do_json_test("tests/jsmoo/cb 2b.json");
}
#[test]
fn op_code_0xCB_0x2C() {
  do_json_test("tests/jsmoo/cb 2c.json");
}
#[test]
fn op_code_0xCB_0x2D() {
  do_json_test("tests/jsmoo/cb 2d.json");
}
#[test]
fn op_code_0xCB_0x2E() {
  do_json_test("tests/jsmoo/cb 2e.json");
}
#[test]
fn op_code_0xCB_0x2F() {
  do_json_test("tests/jsmoo/cb 2f.json");
}
#[test]
fn op_code_0xCB_0x30() {
  do_json_test("tests/jsmoo/cb 30.json");
}
#[test]
fn op_code_0xCB_0x31() {
  do_json_test("tests/jsmoo/cb 31.json");
}
#[test]
fn op_code_0xCB_0x32() {
  do_json_test("tests/jsmoo/cb 32.json");
}
#[test]
fn op_code_0xCB_0x33() {
  do_json_test("tests/jsmoo/cb 33.json");
}
#[test]
fn op_code_0xCB_0x34() {
  do_json_test("tests/jsmoo/cb 34.json");
}
#[test]
fn op_code_0xCB_0x35() {
  do_json_test("tests/jsmoo/cb 35.json");
}
#[test]
fn op_code_0xCB_0x36() {
  do_json_test("tests/jsmoo/cb 36.json");
}
#[test]
fn op_code_0xCB_0x37() {
  do_json_test("tests/jsmoo/cb 37.json");
}
#[test]
fn op_code_0xCB_0x38() {
  do_json_test("tests/jsmoo/cb 38.json");
}
#[test]
fn op_code_0xCB_0x39() {
  do_json_test("tests/jsmoo/cb 39.json");
}
#[test]
fn op_code_0xCB_0x3A() {
  do_json_test("tests/jsmoo/cb 3a.json");
}
#[test]
fn op_code_0xCB_0x3B() {
  do_json_test("tests/jsmoo/cb 3b.json");
}
#[test]
fn op_code_0xCB_0x3C() {
  do_json_test("tests/jsmoo/cb 3c.json");
}
#[test]
fn op_code_0xCB_0x3D() {
  do_json_test("tests/jsmoo/cb 3d.json");
}
#[test]
fn op_code_0xCB_0x3E() {
  do_json_test("tests/jsmoo/cb 3e.json");
}
#[test]
fn op_code_0xCB_0x3F() {
  do_json_test("tests/jsmoo/cb 3f.json");
}
#[test]
fn op_code_0xCB_0x40() {
  do_json_test("tests/jsmoo/cb 40.json");
}
#[test]
fn op_code_0xCB_0x41() {
  do_json_test("tests/jsmoo/cb 41.json");
}
#[test]
fn op_code_0xCB_0x42() {
  do_json_test("tests/jsmoo/cb 42.json");
}
#[test]
fn op_code_0xCB_0x43() {
  do_json_test("tests/jsmoo/cb 43.json");
}
#[test]
fn op_code_0xCB_0x44() {
  do_json_test("tests/jsmoo/cb 44.json");
}
#[test]
fn op_code_0xCB_0x45() {
  do_json_test("tests/jsmoo/cb 45.json");
}
#[test]
fn op_code_0xCB_0x46() {
  do_json_test("tests/jsmoo/cb 46.json");
}
#[test]
fn op_code_0xCB_0x47() {
  do_json_test("tests/jsmoo/cb 47.json");
}
#[test]
fn op_code_0xCB_0x48() {
  do_json_test("tests/jsmoo/cb 48.json");
}
#[test]
fn op_code_0xCB_0x49() {
  do_json_test("tests/jsmoo/cb 49.json");
}
#[test]
fn op_code_0xCB_0x4A() {
  do_json_test("tests/jsmoo/cb 4a.json");
}
#[test]
fn op_code_0xCB_0x4B() {
  do_json_test("tests/jsmoo/cb 4b.json");
}
#[test]
fn op_code_0xCB_0x4C() {
  do_json_test("tests/jsmoo/cb 4c.json");
}
#[test]
fn op_code_0xCB_0x4D() {
  do_json_test("tests/jsmoo/cb 4d.json");
}
#[test]
fn op_code_0xCB_0x4E() {
  do_json_test("tests/jsmoo/cb 4e.json");
}
#[test]
fn op_code_0xCB_0x4F() {
  do_json_test("tests/jsmoo/cb 4f.json");
}
#[test]
fn op_code_0xCB_0x50() {
  do_json_test("tests/jsmoo/cb 50.json");
}
#[test]
fn op_code_0xCB_0x51() {
  do_json_test("tests/jsmoo/cb 51.json");
}
#[test]
fn op_code_0xCB_0x52() {
  do_json_test("tests/jsmoo/cb 52.json");
}
#[test]
fn op_code_0xCB_0x53() {
  do_json_test("tests/jsmoo/cb 53.json");
}
#[test]
fn op_code_0xCB_0x54() {
  do_json_test("tests/jsmoo/cb 54.json");
}
#[test]
fn op_code_0xCB_0x55() {
  do_json_test("tests/jsmoo/cb 55.json");
}
#[test]
fn op_code_0xCB_0x56() {
  do_json_test("tests/jsmoo/cb 56.json");
}
#[test]
fn op_code_0xCB_0x57() {
  do_json_test("tests/jsmoo/cb 57.json");
}
#[test]
fn op_code_0xCB_0x58() {
  do_json_test("tests/jsmoo/cb 58.json");
}
#[test]
fn op_code_0xCB_0x59() {
  do_json_test("tests/jsmoo/cb 59.json");
}
#[test]
fn op_code_0xCB_0x5A() {
  do_json_test("tests/jsmoo/cb 5a.json");
}
#[test]
fn op_code_0xCB_0x5B() {
  do_json_test("tests/jsmoo/cb 5b.json");
}
#[test]
fn op_code_0xCB_0x5C() {
  do_json_test("tests/jsmoo/cb 5c.json");
}
#[test]
fn op_code_0xCB_0x5D() {
  do_json_test("tests/jsmoo/cb 5d.json");
}
#[test]
fn op_code_0xCB_0x5E() {
  do_json_test("tests/jsmoo/cb 5e.json");
}
#[test]
fn op_code_0xCB_0x5F() {
  do_json_test("tests/jsmoo/cb 5f.json");
}
#[test]
fn op_code_0xCB_0x60() {
  do_json_test("tests/jsmoo/cb 60.json");
}
#[test]
fn op_code_0xCB_0x61() {
  do_json_test("tests/jsmoo/cb 61.json");
}
#[test]
fn op_code_0xCB_0x62() {
  do_json_test("tests/jsmoo/cb 62.json");
}
#[test]
fn op_code_0xCB_0x63() {
  do_json_test("tests/jsmoo/cb 63.json");
}
#[test]
fn op_code_0xCB_0x64() {
  do_json_test("tests/jsmoo/cb 64.json");
}
#[test]
fn op_code_0xCB_0x65() {
  do_json_test("tests/jsmoo/cb 65.json");
}
#[test]
fn op_code_0xCB_0x66() {
  do_json_test("tests/jsmoo/cb 66.json");
}
#[test]
fn op_code_0xCB_0x67() {
  do_json_test("tests/jsmoo/cb 67.json");
}
#[test]
fn op_code_0xCB_0x68() {
  do_json_test("tests/jsmoo/cb 68.json");
}
#[test]
fn op_code_0xCB_0x69() {
  do_json_test("tests/jsmoo/cb 69.json");
}
#[test]
fn op_code_0xCB_0x6A() {
  do_json_test("tests/jsmoo/cb 6a.json");
}
#[test]
fn op_code_0xCB_0x6B() {
  do_json_test("tests/jsmoo/cb 6b.json");
}
#[test]
fn op_code_0xCB_0x6C() {
  do_json_test("tests/jsmoo/cb 6c.json");
}
#[test]
fn op_code_0xCB_0x6D() {
  do_json_test("tests/jsmoo/cb 6d.json");
}
#[test]
fn op_code_0xCB_0x6E() {
  do_json_test("tests/jsmoo/cb 6e.json");
}
#[test]
fn op_code_0xCB_0x6F() {
  do_json_test("tests/jsmoo/cb 6f.json");
}
#[test]
fn op_code_0xCB_0x70() {
  do_json_test("tests/jsmoo/cb 70.json");
}
#[test]
fn op_code_0xCB_0x71() {
  do_json_test("tests/jsmoo/cb 71.json");
}
#[test]
fn op_code_0xCB_0x72() {
  do_json_test("tests/jsmoo/cb 72.json");
}
#[test]
fn op_code_0xCB_0x73() {
  do_json_test("tests/jsmoo/cb 73.json");
}
#[test]
fn op_code_0xCB_0x74() {
  do_json_test("tests/jsmoo/cb 74.json");
}
#[test]
fn op_code_0xCB_0x75() {
  do_json_test("tests/jsmoo/cb 75.json");
}
#[test]
fn op_code_0xCB_0x76() {
  do_json_test("tests/jsmoo/cb 76.json");
}
#[test]
fn op_code_0xCB_0x77() {
  do_json_test("tests/jsmoo/cb 77.json");
}
#[test]
fn op_code_0xCB_0x78() {
  do_json_test("tests/jsmoo/cb 78.json");
}
#[test]
fn op_code_0xCB_0x79() {
  do_json_test("tests/jsmoo/cb 79.json");
}
#[test]
fn op_code_0xCB_0x7A() {
  do_json_test("tests/jsmoo/cb 7a.json");
}
#[test]
fn op_code_0xCB_0x7B() {
  do_json_test("tests/jsmoo/cb 7b.json");
}
#[test]
fn op_code_0xCB_0x7C() {
  do_json_test("tests/jsmoo/cb 7c.json");
}
#[test]
fn op_code_0xCB_0x7D() {
  do_json_test("tests/jsmoo/cb 7d.json");
}
#[test]
fn op_code_0xCB_0x7E() {
  do_json_test("tests/jsmoo/cb 7e.json");
}
#[test]
fn op_code_0xCB_0x7F() {
  do_json_test("tests/jsmoo/cb 7f.json");
}
#[test]
fn op_code_0xCB_0x80() {
  do_json_test("tests/jsmoo/cb 80.json");
}
#[test]
fn op_code_0xCB_0x81() {
  do_json_test("tests/jsmoo/cb 81.json");
}
#[test]
fn op_code_0xCB_0x82() {
  do_json_test("tests/jsmoo/cb 82.json");
}
#[test]
fn op_code_0xCB_0x83() {
  do_json_test("tests/jsmoo/cb 83.json");
}
#[test]
fn op_code_0xCB_0x84() {
  do_json_test("tests/jsmoo/cb 84.json");
}
#[test]
fn op_code_0xCB_0x85() {
  do_json_test("tests/jsmoo/cb 85.json");
}
#[test]
fn op_code_0xCB_0x86() {
  do_json_test("tests/jsmoo/cb 86.json");
}
#[test]
fn op_code_0xCB_0x87() {
  do_json_test("tests/jsmoo/cb 87.json");
}
#[test]
fn op_code_0xCB_0x88() {
  do_json_test("tests/jsmoo/cb 88.json");
}
#[test]
fn op_code_0xCB_0x89() {
  do_json_test("tests/jsmoo/cb 89.json");
}
#[test]
fn op_code_0xCB_0x8A() {
  do_json_test("tests/jsmoo/cb 8a.json");
}
#[test]
fn op_code_0xCB_0x8B() {
  do_json_test("tests/jsmoo/cb 8b.json");
}
#[test]
fn op_code_0xCB_0x8C() {
  do_json_test("tests/jsmoo/cb 8c.json");
}
#[test]
fn op_code_0xCB_0x8D() {
  do_json_test("tests/jsmoo/cb 8d.json");
}
#[test]
fn op_code_0xCB_0x8E() {
  do_json_test("tests/jsmoo/cb 8e.json");
}
#[test]
fn op_code_0xCB_0x8F() {
  do_json_test("tests/jsmoo/cb 8f.json");
}
#[test]
fn op_code_0xCB_0x90() {
  do_json_test("tests/jsmoo/cb 90.json");
}
#[test]
fn op_code_0xCB_0x91() {
  do_json_test("tests/jsmoo/cb 91.json");
}
#[test]
fn op_code_0xCB_0x92() {
  do_json_test("tests/jsmoo/cb 92.json");
}
#[test]
fn op_code_0xCB_0x93() {
  do_json_test("tests/jsmoo/cb 93.json");
}
#[test]
fn op_code_0xCB_0x94() {
  do_json_test("tests/jsmoo/cb 94.json");
}
#[test]
fn op_code_0xCB_0x95() {
  do_json_test("tests/jsmoo/cb 95.json");
}
#[test]
fn op_code_0xCB_0x96() {
  do_json_test("tests/jsmoo/cb 96.json");
}
#[test]
fn op_code_0xCB_0x97() {
  do_json_test("tests/jsmoo/cb 97.json");
}
#[test]
fn op_code_0xCB_0x98() {
  do_json_test("tests/jsmoo/cb 98.json");
}
#[test]
fn op_code_0xCB_0x99() {
  do_json_test("tests/jsmoo/cb 99.json");
}
#[test]
fn op_code_0xCB_0x9A() {
  do_json_test("tests/jsmoo/cb 9a.json");
}
#[test]
fn op_code_0xCB_0x9B() {
  do_json_test("tests/jsmoo/cb 9b.json");
}
#[test]
fn op_code_0xCB_0x9C() {
  do_json_test("tests/jsmoo/cb 9c.json");
}
#[test]
fn op_code_0xCB_0x9D() {
  do_json_test("tests/jsmoo/cb 9d.json");
}
#[test]
fn op_code_0xCB_0x9E() {
  do_json_test("tests/jsmoo/cb 9e.json");
}
#[test]
fn op_code_0xCB_0x9F() {
  do_json_test("tests/jsmoo/cb 9f.json");
}
#[test]
fn op_code_0xCB_0xA0() {
  do_json_test("tests/jsmoo/cb a0.json");
}
#[test]
fn op_code_0xCB_0xA1() {
  do_json_test("tests/jsmoo/cb a1.json");
}
#[test]
fn op_code_0xCB_0xA2() {
  do_json_test("tests/jsmoo/cb a2.json");
}
#[test]
fn op_code_0xCB_0xA3() {
  do_json_test("tests/jsmoo/cb a3.json");
}
#[test]
fn op_code_0xCB_0xA4() {
  do_json_test("tests/jsmoo/cb a4.json");
}
#[test]
fn op_code_0xCB_0xA5() {
  do_json_test("tests/jsmoo/cb a5.json");
}
#[test]
fn op_code_0xCB_0xA6() {
  do_json_test("tests/jsmoo/cb a6.json");
}
#[test]
fn op_code_0xCB_0xA7() {
  do_json_test("tests/jsmoo/cb a7.json");
}
#[test]
fn op_code_0xCB_0xA8() {
  do_json_test("tests/jsmoo/cb a8.json");
}
#[test]
fn op_code_0xCB_0xA9() {
  do_json_test("tests/jsmoo/cb a9.json");
}
#[test]
fn op_code_0xCB_0xAA() {
  do_json_test("tests/jsmoo/cb aa.json");
}
#[test]
fn op_code_0xCB_0xAB() {
  do_json_test("tests/jsmoo/cb ab.json");
}
#[test]
fn op_code_0xCB_0xAC() {
  do_json_test("tests/jsmoo/cb ac.json");
}
#[test]
fn op_code_0xCB_0xAD() {
  do_json_test("tests/jsmoo/cb ad.json");
}
#[test]
fn op_code_0xCB_0xAE() {
  do_json_test("tests/jsmoo/cb ae.json");
}
#[test]
fn op_code_0xCB_0xAF() {
  do_json_test("tests/jsmoo/cb af.json");
}
#[test]
fn op_code_0xCB_0xB0() {
  do_json_test("tests/jsmoo/cb b0.json");
}
#[test]
fn op_code_0xCB_0xB1() {
  do_json_test("tests/jsmoo/cb b1.json");
}
#[test]
fn op_code_0xCB_0xB2() {
  do_json_test("tests/jsmoo/cb b2.json");
}
#[test]
fn op_code_0xCB_0xB3() {
  do_json_test("tests/jsmoo/cb b3.json");
}
#[test]
fn op_code_0xCB_0xB4() {
  do_json_test("tests/jsmoo/cb b4.json");
}
#[test]
fn op_code_0xCB_0xB5() {
  do_json_test("tests/jsmoo/cb b5.json");
}
#[test]
fn op_code_0xCB_0xB6() {
  do_json_test("tests/jsmoo/cb b6.json");
}
#[test]
fn op_code_0xCB_0xB7() {
  do_json_test("tests/jsmoo/cb b7.json");
}
#[test]
fn op_code_0xCB_0xB8() {
  do_json_test("tests/jsmoo/cb b8.json");
}
#[test]
fn op_code_0xCB_0xB9() {
  do_json_test("tests/jsmoo/cb b9.json");
}
#[test]
fn op_code_0xCB_0xBA() {
  do_json_test("tests/jsmoo/cb ba.json");
}
#[test]
fn op_code_0xCB_0xBB() {
  do_json_test("tests/jsmoo/cb bb.json");
}
#[test]
fn op_code_0xCB_0xBC() {
  do_json_test("tests/jsmoo/cb bc.json");
}
#[test]
fn op_code_0xCB_0xBD() {
  do_json_test("tests/jsmoo/cb bd.json");
}
#[test]
fn op_code_0xCB_0xBE() {
  do_json_test("tests/jsmoo/cb be.json");
}
#[test]
fn op_code_0xCB_0xBF() {
  do_json_test("tests/jsmoo/cb bf.json");
}
#[test]
fn op_code_0xCB_0xC0() {
  do_json_test("tests/jsmoo/cb c0.json");
}
#[test]
fn op_code_0xCB_0xC1() {
  do_json_test("tests/jsmoo/cb c1.json");
}
#[test]
fn op_code_0xCB_0xC2() {
  do_json_test("tests/jsmoo/cb c2.json");
}
#[test]
fn op_code_0xCB_0xC3() {
  do_json_test("tests/jsmoo/cb c3.json");
}
#[test]
fn op_code_0xCB_0xC4() {
  do_json_test("tests/jsmoo/cb c4.json");
}
#[test]
fn op_code_0xCB_0xC5() {
  do_json_test("tests/jsmoo/cb c5.json");
}
#[test]
fn op_code_0xCB_0xC6() {
  do_json_test("tests/jsmoo/cb c6.json");
}
#[test]
fn op_code_0xCB_0xC7() {
  do_json_test("tests/jsmoo/cb c7.json");
}
#[test]
fn op_code_0xCB_0xC8() {
  do_json_test("tests/jsmoo/cb c8.json");
}
#[test]
fn op_code_0xCB_0xC9() {
  do_json_test("tests/jsmoo/cb c9.json");
}
#[test]
fn op_code_0xCB_0xCA() {
  do_json_test("tests/jsmoo/cb ca.json");
}
#[test]
fn op_code_0xCB_0xCB() {
  do_json_test("tests/jsmoo/cb cb.json");
}
#[test]
fn op_code_0xCB_0xCC() {
  do_json_test("tests/jsmoo/cb cc.json");
}
#[test]
fn op_code_0xCB_0xCD() {
  do_json_test("tests/jsmoo/cb cd.json");
}
#[test]
fn op_code_0xCB_0xCE() {
  do_json_test("tests/jsmoo/cb ce.json");
}
#[test]
fn op_code_0xCB_0xCF() {
  do_json_test("tests/jsmoo/cb cf.json");
}
#[test]
fn op_code_0xCB_0xD0() {
  do_json_test("tests/jsmoo/cb d0.json");
}
#[test]
fn op_code_0xCB_0xD1() {
  do_json_test("tests/jsmoo/cb d1.json");
}
#[test]
fn op_code_0xCB_0xD2() {
  do_json_test("tests/jsmoo/cb d2.json");
}
#[test]
fn op_code_0xCB_0xD3() {
  do_json_test("tests/jsmoo/cb d3.json");
}
#[test]
fn op_code_0xCB_0xD4() {
  do_json_test("tests/jsmoo/cb d4.json");
}
#[test]
fn op_code_0xCB_0xD5() {
  do_json_test("tests/jsmoo/cb d5.json");
}
#[test]
fn op_code_0xCB_0xD6() {
  do_json_test("tests/jsmoo/cb d6.json");
}
#[test]
fn op_code_0xCB_0xD7() {
  do_json_test("tests/jsmoo/cb d7.json");
}
#[test]
fn op_code_0xCB_0xD8() {
  do_json_test("tests/jsmoo/cb d8.json");
}
#[test]
fn op_code_0xCB_0xD9() {
  do_json_test("tests/jsmoo/cb d9.json");
}
#[test]
fn op_code_0xCB_0xDA() {
  do_json_test("tests/jsmoo/cb da.json");
}
#[test]
fn op_code_0xCB_0xDB() {
  do_json_test("tests/jsmoo/cb db.json");
}
#[test]
fn op_code_0xCB_0xDC() {
  do_json_test("tests/jsmoo/cb dc.json");
}
#[test]
fn op_code_0xCB_0xDD() {
  do_json_test("tests/jsmoo/cb dd.json");
}
#[test]
fn op_code_0xCB_0xDE() {
  do_json_test("tests/jsmoo/cb de.json");
}
#[test]
fn op_code_0xCB_0xDF() {
  do_json_test("tests/jsmoo/cb df.json");
}
#[test]
fn op_code_0xCB_0xE0() {
  do_json_test("tests/jsmoo/cb e0.json");
}
#[test]
fn op_code_0xCB_0xE1() {
  do_json_test("tests/jsmoo/cb e1.json");
}
#[test]
fn op_code_0xCB_0xE2() {
  do_json_test("tests/jsmoo/cb e2.json");
}
#[test]
fn op_code_0xCB_0xE3() {
  do_json_test("tests/jsmoo/cb e3.json");
}
#[test]
fn op_code_0xCB_0xE4() {
  do_json_test("tests/jsmoo/cb e4.json");
}
#[test]
fn op_code_0xCB_0xE5() {
  do_json_test("tests/jsmoo/cb e5.json");
}
#[test]
fn op_code_0xCB_0xE6() {
  do_json_test("tests/jsmoo/cb e6.json");
}
#[test]
fn op_code_0xCB_0xE7() {
  do_json_test("tests/jsmoo/cb e7.json");
}
#[test]
fn op_code_0xCB_0xE8() {
  do_json_test("tests/jsmoo/cb e8.json");
}
#[test]
fn op_code_0xCB_0xE9() {
  do_json_test("tests/jsmoo/cb e9.json");
}
#[test]
fn op_code_0xCB_0xEA() {
  do_json_test("tests/jsmoo/cb ea.json");
}
#[test]
fn op_code_0xCB_0xEB() {
  do_json_test("tests/jsmoo/cb eb.json");
}
#[test]
fn op_code_0xCB_0xEC() {
  do_json_test("tests/jsmoo/cb ec.json");
}
#[test]
fn op_code_0xCB_0xED() {
  do_json_test("tests/jsmoo/cb ed.json");
}
#[test]
fn op_code_0xCB_0xEE() {
  do_json_test("tests/jsmoo/cb ee.json");
}
#[test]
fn op_code_0xCB_0xEF() {
  do_json_test("tests/jsmoo/cb ef.json");
}
#[test]
fn op_code_0xCB_0xF0() {
  do_json_test("tests/jsmoo/cb f0.json");
}
#[test]
fn op_code_0xCB_0xF1() {
  do_json_test("tests/jsmoo/cb f1.json");
}
#[test]
fn op_code_0xCB_0xF2() {
  do_json_test("tests/jsmoo/cb f2.json");
}
#[test]
fn op_code_0xCB_0xF3() {
  do_json_test("tests/jsmoo/cb f3.json");
}
#[test]
fn op_code_0xCB_0xF4() {
  do_json_test("tests/jsmoo/cb f4.json");
}
#[test]
fn op_code_0xCB_0xF5() {
  do_json_test("tests/jsmoo/cb f5.json");
}
#[test]
fn op_code_0xCB_0xF6() {
  do_json_test("tests/jsmoo/cb f6.json");
}
#[test]
fn op_code_0xCB_0xF7() {
  do_json_test("tests/jsmoo/cb f7.json");
}
#[test]
fn op_code_0xCB_0xF8() {
  do_json_test("tests/jsmoo/cb f8.json");
}
#[test]
fn op_code_0xCB_0xF9() {
  do_json_test("tests/jsmoo/cb f9.json");
}
#[test]
fn op_code_0xCB_0xFA() {
  do_json_test("tests/jsmoo/cb fa.json");
}
#[test]
fn op_code_0xCB_0xFB() {
  do_json_test("tests/jsmoo/cb fb.json");
}
#[test]
fn op_code_0xCB_0xFC() {
  do_json_test("tests/jsmoo/cb fc.json");
}
#[test]
fn op_code_0xCB_0xFD() {
  do_json_test("tests/jsmoo/cb fd.json");
}
#[test]
fn op_code_0xCB_0xFE() {
  do_json_test("tests/jsmoo/cb fe.json");
}
#[test]
fn op_code_0xCB_0xFF() {
  do_json_test("tests/jsmoo/cb ff.json");
}
