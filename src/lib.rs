use std::collections::VecDeque;

pub trait MemoryBus {
  fn read(&self, address: u16) -> u8;
  fn write(&mut self, address: u16, byte: u8);
}
struct NoBusNeeded;
impl MemoryBus for NoBusNeeded {
  #[inline]
  fn read(&self, _address: u16) -> u8 {
    todo!()
  }
  #[inline]
  fn write(&mut self, _address: u16, _byte: u8) {
    todo!()
  }
}
impl MemoryBus for Vec<u8> {
  #[inline]
  fn read(&self, address: u16) -> u8 {
    self.get(usize::from(address)).copied().unwrap_or(0)
  }
  #[inline]
  fn write(&mut self, address: u16, byte: u8) {
    if let Some(a) = self.get_mut(usize::from(address)) {
      *a = byte;
    }
  }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
struct Flags(u8);
#[allow(unused)]
impl Flags {
  #[inline]
  pub fn zero(&self) -> bool {
    bitfrob::u8_get_bit(7, self.0)
  }
  #[inline]
  pub fn sub(&self) -> bool {
    bitfrob::u8_get_bit(6, self.0)
  }
  #[inline]
  pub fn half(&self) -> bool {
    bitfrob::u8_get_bit(5, self.0)
  }
  #[inline]
  pub fn carry(&self) -> bool {
    bitfrob::u8_get_bit(4, self.0)
  }
  #[inline]
  pub fn set_zero(&mut self, b: bool) {
    self.0 = bitfrob::u8_with_bit(7, self.0, b)
  }
  #[inline]
  pub fn set_sub(&mut self, b: bool) {
    self.0 = bitfrob::u8_with_bit(6, self.0, b)
  }
  #[inline]
  pub fn set_half(&mut self, b: bool) {
    self.0 = bitfrob::u8_with_bit(5, self.0, b)
  }
  #[inline]
  pub fn set_carry(&mut self, b: bool) {
    self.0 = bitfrob::u8_with_bit(4, self.0, b)
  }
}

#[derive(Debug, Clone)]
pub struct SM83 {
  af: u16,
  bc: u16,
  de: u16,
  hl: u16,
  sp: u16,
  pc: u16,
  imm: u16,
  queue: VecDeque<Action>,
}
impl Default for SM83 {
  #[inline]
  fn default() -> Self {
    let mut queue = VecDeque::default();
    queue.extend([Nop].into_iter());
    Self { af: 0, bc: 0, de: 0, hl: 0, sp: 0, pc: 0, imm: 0, queue }
  }
}
// get/set by enum value
impl SM83 {
  #[inline]
  pub fn get_r8(&self, r8: Reg8, bus: &impl MemoryBus) -> u8 {
    match r8 {
      Reg8::A => bytemuck::bytes_of(&self.af)[1],
      Reg8::F => bytemuck::bytes_of(&self.af)[0],
      Reg8::B => bytemuck::bytes_of(&self.bc)[1],
      Reg8::C => bytemuck::bytes_of(&self.bc)[0],
      Reg8::D => bytemuck::bytes_of(&self.de)[1],
      Reg8::E => bytemuck::bytes_of(&self.de)[0],
      Reg8::H => bytemuck::bytes_of(&self.hl)[1],
      Reg8::L => bytemuck::bytes_of(&self.hl)[0],
      Reg8::SPL => bytemuck::bytes_of(&self.sp)[1],
      Reg8::SPH => bytemuck::bytes_of(&self.sp)[0],
      Reg8::PCL => bytemuck::bytes_of(&self.pc)[1],
      Reg8::PCH => bytemuck::bytes_of(&self.pc)[0],
      Reg8::IMML => bytemuck::bytes_of(&self.imm)[1],
      Reg8::IMMH => bytemuck::bytes_of(&self.imm)[0],
      Reg8::HLA => bus.read(self.hl),
    }
  }
  #[inline]
  pub fn set_r8(&mut self, r8: Reg8, u: u8, bus: &mut impl MemoryBus) {
    match r8 {
      Reg8::A => bytemuck::bytes_of_mut(&mut self.af)[1] = u,
      Reg8::F => bytemuck::bytes_of_mut(&mut self.af)[0] = u,
      Reg8::B => bytemuck::bytes_of_mut(&mut self.bc)[1] = u,
      Reg8::C => bytemuck::bytes_of_mut(&mut self.bc)[0] = u,
      Reg8::D => bytemuck::bytes_of_mut(&mut self.de)[1] = u,
      Reg8::E => bytemuck::bytes_of_mut(&mut self.de)[0] = u,
      Reg8::H => bytemuck::bytes_of_mut(&mut self.hl)[1] = u,
      Reg8::L => bytemuck::bytes_of_mut(&mut self.hl)[0] = u,
      Reg8::SPL => bytemuck::bytes_of_mut(&mut self.sp)[1] = u,
      Reg8::SPH => bytemuck::bytes_of_mut(&mut self.sp)[0] = u,
      Reg8::PCL => bytemuck::bytes_of_mut(&mut self.pc)[1] = u,
      Reg8::PCH => bytemuck::bytes_of_mut(&mut self.pc)[0] = u,
      Reg8::IMML => bytemuck::bytes_of_mut(&mut self.imm)[1] = u,
      Reg8::IMMH => bytemuck::bytes_of_mut(&mut self.imm)[0] = u,
      Reg8::HLA => bus.write(self.hl, u),
    }
  }
  #[inline]
  pub fn get_r16(&self, r16: Reg16) -> u16 {
    match r16 {
      Reg16::AF => self.af,
      Reg16::BC => self.bc,
      Reg16::DE => self.de,
      Reg16::HL => self.hl,
      Reg16::SP => self.sp,
      Reg16::PC => self.pc,
      Reg16::IMM => self.imm,
    }
  }
  #[inline]
  pub fn set_r16(&mut self, r16: Reg16, u: u16) {
    match r16 {
      Reg16::AF => self.af = u,
      Reg16::BC => self.bc = u,
      Reg16::DE => self.de = u,
      Reg16::HL => self.hl = u,
      Reg16::SP => self.sp = u,
      Reg16::PC => self.pc = u,
      Reg16::IMM => self.imm = u,
    }
  }
}
// get/set per field
impl SM83 {
  #[inline]
  pub fn a(&self) -> u8 {
    self.get_r8(Reg8::A, &NoBusNeeded)
  }
  #[inline]
  pub fn f(&self) -> u8 {
    self.get_r8(Reg8::F, &NoBusNeeded)
  }
  #[inline]
  pub fn b(&self) -> u8 {
    self.get_r8(Reg8::B, &NoBusNeeded)
  }
  #[inline]
  pub fn c(&self) -> u8 {
    self.get_r8(Reg8::C, &NoBusNeeded)
  }
  #[inline]
  pub fn d(&self) -> u8 {
    self.get_r8(Reg8::D, &NoBusNeeded)
  }
  #[inline]
  pub fn e(&self) -> u8 {
    self.get_r8(Reg8::E, &NoBusNeeded)
  }
  #[inline]
  pub fn h(&self) -> u8 {
    self.get_r8(Reg8::H, &NoBusNeeded)
  }
  #[inline]
  pub fn l(&self) -> u8 {
    self.get_r8(Reg8::L, &NoBusNeeded)
  }
  #[inline]
  pub fn set_a(&mut self, u: u8) {
    self.set_r8(Reg8::A, u, &mut NoBusNeeded)
  }
  #[inline]
  pub fn set_f(&mut self, u: u8) {
    self.set_r8(Reg8::F, u, &mut NoBusNeeded)
  }
  #[inline]
  pub fn set_b(&mut self, u: u8) {
    self.set_r8(Reg8::B, u, &mut NoBusNeeded)
  }
  #[inline]
  pub fn set_c(&mut self, u: u8) {
    self.set_r8(Reg8::C, u, &mut NoBusNeeded)
  }
  #[inline]
  pub fn set_d(&mut self, u: u8) {
    self.set_r8(Reg8::D, u, &mut NoBusNeeded)
  }
  #[inline]
  pub fn set_e(&mut self, u: u8) {
    self.set_r8(Reg8::E, u, &mut NoBusNeeded)
  }
  #[inline]
  pub fn set_h(&mut self, u: u8) {
    self.set_r8(Reg8::H, u, &mut NoBusNeeded)
  }
  #[inline]
  pub fn set_l(&mut self, u: u8) {
    self.set_r8(Reg8::L, u, &mut NoBusNeeded)
  }
  #[inline]
  pub fn pc(&self) -> u16 {
    self.get_r16(Reg16::PC)
  }
  #[inline]
  pub fn sp(&self) -> u16 {
    self.get_r16(Reg16::SP)
  }
  #[inline]
  pub fn set_pc(&mut self, u: u16) {
    self.set_r16(Reg16::PC, u)
  }
  #[inline]
  pub fn set_sp(&mut self, u: u16) {
    self.set_r16(Reg16::SP, u)
  }
}

impl SM83 {
  #[inline]
  pub fn m_cycle(&mut self, bus: &mut impl MemoryBus) {
    let action = self.queue.pop_front().unwrap();
    println!("Taking Action: {action:?}");
    match action {
      Nop => (),
      Read(r8, r16, i) => {
        let address = self.get_r16(r16);
        let byte = bus.read(address);
        println!("bus[{address}] == {byte}");
        self.set_r8(r8, byte, bus);
        let new_address = address.wrapping_add(i16::from(i) as u16);
        self.set_r16(r16, new_address);
      }
      Write(r16, r8, i) => {
        let address = self.get_r16(r16);
        let byte = self.get_r8(r8, bus);
        bus.write(address, byte);
        let new_address = address.wrapping_add(i16::from(i) as u16);
        self.set_r16(r16, new_address);
      }
      Delta16(r16, i) => {
        let r = self.get_r16(r16);
        let new_r = r.wrapping_add(i16::from(i) as u16);
        self.set_r16(r16, new_r);
      }
      Delta8(r8, i) => {
        let r = self.get_r8(r8, bus);
        let new_r = r.wrapping_add(i as u8);
        dbg!(r, new_r);
        self.set_r8(r8, new_r, bus);
        let mut f = Flags(self.f());
        f.set_zero(new_r == 0);
        f.set_sub(i < 0);
        f.set_half((r & 0b11110000) != (new_r & 0b11110000));
        self.set_f(f.0);
      }
    }

    if self.queue.is_empty() {
      println!("Queue Is Empty.");
      let address = self.pc;
      let op = bus.read(address);
      println!("Addr:{address}, Op:0x{op:02X}");
      let actions = OP_TABLE.get(usize::from(op)).copied().unwrap_or(&[Nop]);
      debug_assert!(!actions.is_empty());
      println!("Adding To Queue: {actions:?}");
      self.queue.extend(actions.iter().copied());
      self.pc = self.pc.wrapping_add(1);
    }
    debug_assert!(!self.queue.is_empty());
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Reg8 {
  A,
  F,
  B,
  C,
  D,
  E,
  H,
  L,
  SPL,
  SPH,
  PCL,
  PCH,
  IMML,
  IMMH,
  /// Indicates the byte at the HL address
  HLA,
}
use Reg8::*;

#[derive(Debug, Clone, Copy)]
pub enum Reg16 {
  AF,
  BC,
  DE,
  HL,
  SP,
  PC,
  IMM,
}
use Reg16::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum Action {
  /// No-operation
  #[default]
  Nop,

  /// `reg8 = *reg16; reg16 += i;`
  ///
  /// Read the address then (wrapping) offset by the delta.
  Read(Reg8, Reg16, i8),

  /// `*reg16 = reg8; reg16 += i;`
  ///
  /// Write the address then (wrapping) offset by the delta.
  Write(Reg16, Reg8, i8),

  /// Adjust a 16-bit register by an offset.
  Delta16(Reg16, i8),

  /// (Flags) Adjust an 8-bit register by an offset.
  Delta8(Reg8, i8),
}
use Action::*;
/// `Read(r8, PC, 1)`, Read PC into a Reg8, then offset PC by 1.
#[allow(bad_style)]
const fn ReadPC(r8: Reg8) -> Action {
  Read(r8, PC, 1)
}
#[allow(bad_style)]
const fn Inc16(r16: Reg16) -> Action {
  Delta16(r16, 1)
}
#[allow(bad_style)]
const fn Inc8(r8: Reg8) -> Action {
  Delta8(r8, 1)
}
#[allow(bad_style)]
const fn Dec8(r8: Reg8) -> Action {
  Delta8(r8, -1)
}

static OP_TABLE: &[&[Action]] = &[
  // 0x00 series
  &[Nop],                       /* NOP */
  &[ReadPC(C), ReadPC(B), Nop], /* LD BC, imm16 */
  &[Write(BC, A, 0), Nop],      /* LD [BC], A */
  &[Inc16(BC), Nop],            /* INC BC */
  &[Inc8(B)],                   /* INC B */
  &[Dec8(B)],                   /* DEC B */
  &[ReadPC(B), Nop],            /* LD B, n8 */
  &[Nop],                       /* RLCA */
  &[Nop],                       /* LD [a16], SP */
  &[Nop],                       /* ADD HL, BC */
  &[Nop],                       /* LD A, [BC] */
  &[Nop],                       /* DEC BC */
  &[Nop],                       /* INC C */
  &[Nop],                       /* DEC C */
  &[Nop],                       /* LD C, n8 */
  &[Nop],                       /* RRCA */
];
