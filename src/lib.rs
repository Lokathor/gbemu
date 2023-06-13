#![allow(unused_parens)]

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
  const HIGH: usize = cfg!(target_endian = "little") as usize;
  const LOW: usize = (!cfg!(target_endian = "little")) as usize;
  #[inline]
  pub fn get_r8(&self, r8: Reg8, bus: &impl MemoryBus) -> u8 {
    use bytemuck::bytes_of;
    match r8 {
      Reg8::A => bytes_of(&self.af)[Self::HIGH],
      Reg8::F => bytes_of(&self.af)[Self::LOW],
      Reg8::B => bytes_of(&self.bc)[Self::HIGH],
      Reg8::C => bytes_of(&self.bc)[Self::LOW],
      Reg8::D => bytes_of(&self.de)[Self::HIGH],
      Reg8::E => bytes_of(&self.de)[Self::LOW],
      Reg8::H => bytes_of(&self.hl)[Self::HIGH],
      Reg8::L => bytes_of(&self.hl)[Self::LOW],
      Reg8::SPH => bytes_of(&self.sp)[Self::HIGH],
      Reg8::SPL => bytes_of(&self.sp)[Self::LOW],
      Reg8::PCH => bytes_of(&self.pc)[Self::HIGH],
      Reg8::PCL => bytes_of(&self.pc)[Self::LOW],
      Reg8::ImmH => bytes_of(&self.imm)[Self::HIGH],
      Reg8::ImmL => bytes_of(&self.imm)[Self::LOW],
      Reg8::HLA => bus.read(self.hl),
    }
  }
  #[inline]
  pub fn set_r8(&mut self, r8: Reg8, u: u8, bus: &mut impl MemoryBus) {
    use bytemuck::bytes_of_mut;
    match r8 {
      Reg8::A => bytes_of_mut(&mut self.af)[Self::HIGH] = u,
      Reg8::F => bytes_of_mut(&mut self.af)[Self::LOW] = u,
      Reg8::B => bytes_of_mut(&mut self.bc)[Self::HIGH] = u,
      Reg8::C => bytes_of_mut(&mut self.bc)[Self::LOW] = u,
      Reg8::D => bytes_of_mut(&mut self.de)[Self::HIGH] = u,
      Reg8::E => bytes_of_mut(&mut self.de)[Self::LOW] = u,
      Reg8::H => bytes_of_mut(&mut self.hl)[Self::HIGH] = u,
      Reg8::L => bytes_of_mut(&mut self.hl)[Self::LOW] = u,
      Reg8::SPH => bytes_of_mut(&mut self.sp)[Self::HIGH] = u,
      Reg8::SPL => bytes_of_mut(&mut self.sp)[Self::LOW] = u,
      Reg8::PCH => bytes_of_mut(&mut self.pc)[Self::HIGH] = u,
      Reg8::PCL => bytes_of_mut(&mut self.pc)[Self::LOW] = u,
      Reg8::ImmH => bytes_of_mut(&mut self.imm)[Self::HIGH] = u,
      Reg8::ImmL => bytes_of_mut(&mut self.imm)[Self::LOW] = u,
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
      Reg16::Imm => self.imm,
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
      Reg16::Imm => self.imm = u,
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
  pub fn imm_l(&self) -> u8 {
    self.get_r8(Reg8::ImmL, &NoBusNeeded)
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
  pub fn hl(&self) -> u16 {
    self.get_r16(Reg16::HL)
  }
  #[inline]
  pub fn set_pc(&mut self, u: u16) {
    self.set_r16(Reg16::PC, u)
  }
  #[inline]
  pub fn set_sp(&mut self, u: u16) {
    self.set_r16(Reg16::SP, u)
  }
  #[inline]
  pub fn set_hl(&mut self, u: u16) {
    self.set_r16(Reg16::HL, u)
  }

  #[inline]
  pub fn f_zero(&self) -> bool {
    bitfrob::u8_get_bit(7, self.f())
  }
  #[inline]
  pub fn f_sub(&self) -> bool {
    bitfrob::u8_get_bit(6, self.f())
  }
  #[inline]
  pub fn f_half(&self) -> bool {
    bitfrob::u8_get_bit(5, self.f())
  }
  #[inline]
  pub fn f_carry(&self) -> bool {
    bitfrob::u8_get_bit(4, self.f())
  }
  #[inline]
  pub fn set_f_zero(&mut self, b: bool) {
    self.set_f(bitfrob::u8_with_bit(7, self.f(), b))
  }
  #[inline]
  pub fn set_f_sub(&mut self, b: bool) {
    self.set_f(bitfrob::u8_with_bit(6, self.f(), b))
  }
  #[inline]
  pub fn set_f_half(&mut self, b: bool) {
    self.set_f(bitfrob::u8_with_bit(5, self.f(), b))
  }
  #[inline]
  pub fn set_f_carry(&mut self, b: bool) {
    self.set_f(bitfrob::u8_with_bit(4, self.f(), b))
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
        println!("> bus[{address}] is {byte}");
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
        self.set_f_zero(new_r == 0);
        self.set_f_sub(i < 0);
        self.set_f_half((r & 0xF0) != (new_r & 0xF0));
      }
      RotateCyClearZ(leftward) => {
        let a = self.a();
        let new_a = if leftward { a.rotate_left(1) } else { a.rotate_right(1) };
        self.set_a(new_a);
        self.set_f_zero(false);
        self.set_f_sub(false);
        self.set_f_half(false);
        self.set_f_carry(if leftward { (a as i8) < 0 } else { (a & 1) != 0 });
      }
      RotateClearZ(leftward) => {
        let a = self.a();
        let new_a = if leftward {
          a << 1 | self.f_carry() as u8
        } else {
          a >> 1 | (self.f_carry() as u8) << 7
        };
        self.set_a(new_a);
        self.set_f_zero(false);
        self.set_f_sub(false);
        self.set_f_half(false);
        self.set_f_carry(if leftward { (a as i8) < 0 } else { (a & 1) != 0 });
      }
      AddHL(rhs_reg) => {
        let hl = self.hl();
        let rhs = self.get_r16(rhs_reg);
        let (new_hl, carried) = hl.overflowing_add(rhs);
        self.set_hl(new_hl);
        self.set_f_sub(false);
        self.set_f_half((new_hl & 0xFFF) < (rhs & 0xFFF));
        self.set_f_carry(carried);
      }
      JpRel(cond) => {
        let is_cond = match cond {
          Al => true,
          Ze => self.f_zero(),
          NZ => !self.f_zero(),
          Cy => self.f_carry(),
          NC => !self.f_carry(),
        };
        if is_cond {
          let pc = self.pc();
          let delta: i8 = self.imm_l() as i8;
          let new_pc = pc.wrapping_add_signed(i16::from(delta));
          self.set_pc(new_pc);
        } else {
          debug_assert_eq!(self.queue.len(), 1);
          self.queue.clear();
        }
      }
      DecimalAdjustA => {
        let a = self.a();
        let sub = self.f_sub();
        // step 1
        if !sub && a >= 0x9A {
          self.set_f_carry(true);
        }
        // step 2
        if !sub && (a & 0xF) >= 0x0A {
          self.set_f_half(true);
        }
        // step 3
        let adj_h = if self.f_half() { 0x06 } else { 0 };
        let adj_c = if self.f_carry() { 0x60 } else { 0 };
        let adjustment = adj_h | adj_c;
        // step 4
        let new_a = if sub { a.wrapping_sub(adjustment) } else { a.wrapping_add(adjustment) };
        // step 5
        self.set_a(new_a);
        self.set_f_zero(new_a == 0);
        self.set_f_half(false);
      }
      ComplimentA => {
        let a = self.a();
        let new_a = a ^ 0xFF;
        self.set_a(new_a);
        self.set_f_sub(true);
        self.set_f_half(true);
      }
      ComplimentCarryFlag => {
        self.set_f_sub(false);
        self.set_f_half(false);
        self.set_f_carry(!self.f_carry());
      }
      SetCarryFlag => {
        self.set_f_sub(false);
        self.set_f_half(false);
        self.set_f_carry(true);
      }
    }

    if self.queue.is_empty() {
      let address = self.pc;
      let op = bus.read(address);
      let actions = OP_TABLE.get(usize::from(op)).copied().unwrap_or(&[Nop]);
      debug_assert!(!actions.is_empty());
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
  ImmL,
  ImmH,
  /// Indicates the byte at the HL address
  //#[cfg(FALSE)]
  //TODO: delete this? I think it's always handled by Action list differences.
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
  Imm,
}
use Reg16::*;

#[derive(Debug, Clone, Copy)]
pub enum Cond {
  Al,
  Ze,
  NZ,
  Cy,
  NC,
}
use Cond::*;

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

  /*
   * TODO: we can merge the cycle and non-cycle actions by
   * just having a flag for if we want to use the real
   * carry value or make it act like it's 0.
   */
  /// (Flags) Rotate (left?) "Cycle" A, Then Clear Z
  RotateCyClearZ(bool),

  /// (Flags) Rotate (left?) A, Then Clear Z
  RotateClearZ(bool),

  /// (Flags) HL += right;
  AddHL(Reg16),

  /// Conditional Jump by an `i8` amount loaded into `ImmL`
  /// * On a **failed** jump condition, flush the queue.
  JpRel(Cond),

  /// (Flags)
  DecimalAdjustA,

  /// (Flags)
  ComplimentA,

  /// (Flags)
  ComplimentCarryFlag,

  /// (Flags)
  SetCarryFlag,
}
use Action::*;
/// `Read(r8, PC, 1)`, Read PC into a Reg8, then offset PC by 1.
#[allow(bad_style)]
const fn RePC(r8: Reg8) -> Action {
  Read(r8, PC, 1)
}
/// `Write(r16, r8, 0)`
#[allow(bad_style)]
const fn Wr0(r16: Reg16, r8: Reg8) -> Action {
  Write(r16, r8, 0)
}
/// `Write(r16, r8, 1)`
#[allow(bad_style)]
const fn WrPP(r16: Reg16, r8: Reg8) -> Action {
  Write(r16, r8, 1)
}
#[allow(bad_style)]
const fn Inc16(r16: Reg16) -> Action {
  Delta16(r16, 1)
}
#[allow(bad_style)]
const fn Dec16(r16: Reg16) -> Action {
  Delta16(r16, -1)
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
  &[Nop],                                                         /* NOP */
  &[RePC(C), RePC(B), Nop],                                       /* LD BC, n16 */
  &[Wr0(BC, A), Nop],                                             /* LD [BC], A */
  &[Inc16(BC), Nop],                                              /* INC BC */
  &[Inc8(B)],                                                     /* INC B */
  &[Dec8(B)],                                                     /* DEC B */
  &[RePC(B), Nop],                                                /* LD B, n8 */
  &[RotateCyClearZ(true)],                                        /* RLCA */
  &[RePC(ImmL), RePC(ImmH), WrPP(Imm, SPL), WrPP(Imm, SPH), Nop], /* LD [a16], SP */
  &[AddHL(BC), Nop],                                              /* ADD HL, BC */
  &[Read(A, BC, 0), Nop],                                         /* LD A, [BC] */
  &[Dec16(BC), Nop],                                              /* DEC BC */
  &[Inc8(C)],                                                     /* INC C */
  &[Dec8(C)],                                                     /* DEC C */
  &[RePC(C), Nop],                                                /* LD C, n8 */
  &[RotateCyClearZ(false)],                                       /* RRCA */
  // 0x10 series
  &[Nop /* GBC Only (kinda) */], /* STOP */
  &[RePC(E), RePC(D), Nop],      /* LD DE, n16 */
  &[Wr0(DE, A), Nop],            /* LD [DE], A */
  &[Inc16(DE), Nop],             /* INC DE */
  &[Inc8(D)],                    /* INC D */
  &[Dec8(D)],                    /* DEC D */
  &[RePC(D), Nop],               /* LD D, n8 */
  &[RotateClearZ(true)],         /* RLA */
  &[RePC(ImmL), JpRel(Al), Nop], /* JR e8 */
  &[AddHL(DE), Nop],             /* ADD HL, DE */
  &[Read(A, DE, 0), Nop],        /* LD A, [DE] */
  &[Dec16(DE), Nop],             /* DEC DE */
  &[Inc8(E)],                    /* INC E */
  &[Dec8(E)],                    /* DEC E */
  &[RePC(E), Nop],               /* LD E, n8 */
  &[RotateClearZ(false)],        /* RRA */
  // 0x20 series
  &[RePC(ImmL), JpRel(NZ), Nop], /* JR NZ, e8 */
  &[RePC(L), RePC(H), Nop],      /* LD HL, n16 */
  &[Write(HL, A, 1), Nop],       /* LD [HL+], A */
  &[Inc16(HL), Nop],             /* INC HL */
  &[Inc8(H)],                    /* INC H */
  &[Dec8(H)],                    /* DEC H */
  &[RePC(H), Nop],               /* LD H, n8 */
  &[DecimalAdjustA],             /* DAA */
  &[RePC(ImmL), JpRel(Ze), Nop], /* JR Z, e8 */
  &[AddHL(HL), Nop],             /* ADD HL, HL */
  &[Read(A, HL, 1), Nop],        /* LD A, [HL+] */
  &[Dec16(HL), Nop],             /* DEC HL */
  &[Inc8(L)],                    /* INC L */
  &[Dec8(L)],                    /* DEC L */
  &[RePC(L), Nop],               /* LD L, n8 */
  &[ComplimentA],                /* CPL */
  // 0x30 series
  &[RePC(ImmL), JpRel(NC), Nop],                   /* JR NC, e8 */
  &[RePC(SPL), RePC(SPH), Nop],                    /* LD SP, n16 */
  &[Write(HL, A, -1), Nop],                        /* LD [HL-], A */
  &[Inc16(SP), Nop],                               /* INC SP */
  &[Read(ImmL, HL, 0), Inc8(ImmL), Wr0(HL, ImmL)], /* INC [HL] */
  &[Read(ImmL, HL, 0), Dec8(ImmL), Wr0(HL, ImmL)], /* DEC [HL] */
  &[RePC(ImmL), Wr0(HL, ImmL), Nop],               /* LD [HL], n8 */
  &[SetCarryFlag],                                 /* SCF */
  &[RePC(ImmL), JpRel(Cy), Nop],                   /* JR C, e8 */
  &[AddHL(SP), Nop],                               /* ADD HL, SP */
  &[Read(A, HL, -1), Nop],                         /* LD A, [HL-] */
  &[Dec16(SP), Nop],                               /* DEC SP */
  &[Inc8(A)],                                      /* INC A */
  &[Dec8(A)],                                      /* DEC A */
  &[RePC(A), Nop],                                 /* LD A, n8 */
  &[ComplimentCarryFlag],                          /* CCF */
];
