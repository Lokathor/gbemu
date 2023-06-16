use gbemu::{
  cpu::{MemoryBus, SM83},
  mbc1::MBC1,
};

struct DebugMemoryBus {
  pub mbc: Box<dyn MemoryBus>,
  pub other: Vec<u8>,
  pub always_vblank: bool,
  pub out_buf: Vec<u8>,
}
impl MemoryBus for DebugMemoryBus {
  fn read(&self, address: u16) -> u8 {
    match address {
      0xFF44 if self.always_vblank => 144,
      0x0000..=0x7FFF => self.mbc.read(address),
      0xA000..=0xBFFF => self.mbc.read(address),
      _ => self.other.read(address),
    }
  }

  fn write(&mut self, address: u16, byte: u8) {
    match address {
      0x0000..=0x7FFF => self.mbc.write(address, byte),
      0xA000..=0xBFFF => self.mbc.write(address, byte),
      _ => self.other.write(address, byte),
    };
    if address == 0xFF01 {
      self.out_buf.push(byte);
    }
  }
}

fn run_blargg_test(filename: &str) {
  let file_bytes = std::fs::read(filename).unwrap();
  assert_eq!(file_bytes[0x147], 0x01);
  let mbc1 = MBC1::new(&file_bytes, None).unwrap();
  let mut bus = DebugMemoryBus {
    mbc: Box::new(mbc1),
    other: vec![0_u8; 0x1_0000],
    always_vblank: true,
    out_buf: vec![],
  };
  let mut cpu = SM83::default();
  cpu.set_pc(0x100);
  for _ in 0..20_000_000 {
    cpu.m_cycle(&mut bus);
  }
  let msg = core::str::from_utf8(&bus.out_buf).unwrap();
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