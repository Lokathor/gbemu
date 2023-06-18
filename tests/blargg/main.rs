use gbemu::{
  cpu::{CpuMode, MemoryBus, SM83},
  mbc1::MBC1,
  ExternParts,
};

fn run_blargg_test(filename: &str) {
  let file_bytes = std::fs::read(filename).unwrap();
  assert_eq!(file_bytes[0x147], 0x01);
  let mbc1 = MBC1::new(&file_bytes, None).unwrap();
  let mut parts = ExternParts::from_cart(Box::new(mbc1));
  parts.serial_log = Some(vec![]);
  let mut cpu = SM83::default();
  cpu.set_pc(0x100);
  let mut cpu_mode = CpuMode::Normal;
  for _ in 0..20_000_000 {
    parts.m_cycle();
    match cpu_mode {
      CpuMode::Normal => cpu_mode = cpu.m_cycle(&mut parts),
      CpuMode::Halted => {
        // wake up only once an interrupt is ready
        if parts.check_ie_and_if() != 0 {
          cpu_mode = cpu.m_cycle(&mut parts)
        }
      }
      CpuMode::Stopped => todo!(),
    }
    #[cfg(FALSE)]
    if cpu_mode != CpuMode::Normal && (!cpu.ime() || parts.read(0xFF0F) == 0) {
      // If we halt or stop and an interrupt can't be triggered then we've
      // locked the CPU forever. The test shouldn't ever do this, so it's time
      // to panic.
      panic!("system locked with IME off and cpu_mode={cpu_mode:?}");
    }
  }
  let msg = core::str::from_utf8(parts.serial_log.as_deref().unwrap_or_default()).unwrap();
  if !msg.contains("Passed") {
    panic!("{msg}");
  }
}

#[test]
fn test01_special() {
  run_blargg_test("tests/blargg/01-special.gb");
}

#[test]
fn test02_interrupts() {
  run_blargg_test("tests/blargg/02-interrupts.gb");
}

#[test]
fn test03_op_sp_hl() {
  run_blargg_test("tests/blargg/03-op sp,hl.gb");
}

#[test]
fn test04_op_r_imm() {
  run_blargg_test("tests/blargg/04-op r,imm.gb");
}

#[test]
fn test05_op_rp() {
  run_blargg_test("tests/blargg/05-op rp.gb");
}

#[test]
fn test06_ld_r_r() {
  run_blargg_test("tests/blargg/06-ld r,r.gb");
}

#[test]
fn test07_jr_jp_call_ret_rst() {
  run_blargg_test("tests/blargg/07-jr,jp,call,ret,rst.gb");
}

#[test]
fn test08_misc_instrs() {
  run_blargg_test("tests/blargg/08-misc instrs.gb");
}

#[test]
fn test09_op_r_r() {
  run_blargg_test("tests/blargg/09-op r,r.gb");
}

#[test]
fn test10_bit_ops() {
  run_blargg_test("tests/blargg/10-bit ops.gb");
}

#[test]
fn test11_op_a_hl() {
  run_blargg_test("tests/blargg/11-op a,(hl).gb");
}
