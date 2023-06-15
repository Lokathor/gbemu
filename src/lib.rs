#![allow(unused_parens)]

use std::collections::VecDeque;

pub trait MemoryBus {
  fn read(&self, address: u16) -> u8;
  fn write(&mut self, address: u16, byte: u8);
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
  irq_enabled: bool,
}
impl Default for SM83 {
  #[inline]
  fn default() -> Self {
    let mut queue = VecDeque::default();
    queue.extend([Nop].into_iter());
    Self { af: 0, bc: 0, de: 0, hl: 0, sp: 0, pc: 0, imm: 0, queue, irq_enabled: false }
  }
}
// get/set by enum value
impl SM83 {
  const HIGH: usize = cfg!(target_endian = "little") as usize;
  const LOW: usize = (!cfg!(target_endian = "little")) as usize;
  #[inline]
  pub fn get_r8(&self, r8: Reg8) -> u8 {
    use bytemuck::bytes_of;
    match r8 {
      A => bytes_of(&self.af)[Self::HIGH],
      F => bytes_of(&self.af)[Self::LOW],
      B => bytes_of(&self.bc)[Self::HIGH],
      C => bytes_of(&self.bc)[Self::LOW],
      D => bytes_of(&self.de)[Self::HIGH],
      E => bytes_of(&self.de)[Self::LOW],
      H => bytes_of(&self.hl)[Self::HIGH],
      L => bytes_of(&self.hl)[Self::LOW],
      SPH => bytes_of(&self.sp)[Self::HIGH],
      SPL => bytes_of(&self.sp)[Self::LOW],
      PCH => bytes_of(&self.pc)[Self::HIGH],
      PCL => bytes_of(&self.pc)[Self::LOW],
      ImmH => bytes_of(&self.imm)[Self::HIGH],
      ImmL => bytes_of(&self.imm)[Self::LOW],
    }
  }
  #[inline]
  pub fn set_r8(&mut self, r8: Reg8, u: u8) {
    use bytemuck::bytes_of_mut;
    match r8 {
      A => bytes_of_mut(&mut self.af)[Self::HIGH] = u,
      F => bytes_of_mut(&mut self.af)[Self::LOW] = u & 0xF0,
      B => bytes_of_mut(&mut self.bc)[Self::HIGH] = u,
      C => bytes_of_mut(&mut self.bc)[Self::LOW] = u,
      D => bytes_of_mut(&mut self.de)[Self::HIGH] = u,
      E => bytes_of_mut(&mut self.de)[Self::LOW] = u,
      H => bytes_of_mut(&mut self.hl)[Self::HIGH] = u,
      L => bytes_of_mut(&mut self.hl)[Self::LOW] = u,
      SPH => bytes_of_mut(&mut self.sp)[Self::HIGH] = u,
      SPL => bytes_of_mut(&mut self.sp)[Self::LOW] = u,
      PCH => bytes_of_mut(&mut self.pc)[Self::HIGH] = u,
      PCL => bytes_of_mut(&mut self.pc)[Self::LOW] = u,
      ImmH => bytes_of_mut(&mut self.imm)[Self::HIGH] = u,
      ImmL => bytes_of_mut(&mut self.imm)[Self::LOW] = u,
    }
  }
  #[inline]
  pub fn get_r16(&self, r16: Reg16) -> u16 {
    match r16 {
      AF => self.af,
      BC => self.bc,
      DE => self.de,
      HL => self.hl,
      SP => self.sp,
      PC => self.pc,
      Imm => self.imm,
      HiPg(r8) => 0xFF00_u16.wrapping_add(u16::from(self.get_r8(r8))),
    }
  }
  #[inline]
  pub fn set_r16(&mut self, r16: Reg16, u: u16) {
    match r16 {
      AF => self.af = u & 0xFFF0,
      BC => self.bc = u,
      DE => self.de = u,
      HL => self.hl = u,
      SP => self.sp = u,
      PC => self.pc = u,
      Imm => self.imm = u,
      HiPg(r8) => self.set_r8(r8, u as u8),
    }
  }
  #[inline]
  pub fn is_cond(&self, cond: Cond) -> bool {
    match cond {
      Al => true,
      Ze => self.f_zero(),
      NZ => !self.f_zero(),
      Cy => self.f_carry(),
      NC => !self.f_carry(),
    }
  }
  #[inline]
  pub fn irq_enabled(&self) -> bool {
    self.irq_enabled
  }
  #[inline]
  pub fn set_irq_enabled(&mut self, irq_enabled: bool) {
    self.irq_enabled = irq_enabled;
  }
}
// get/set per field
impl SM83 {
  #[inline]
  pub fn a(&self) -> u8 {
    self.get_r8(Reg8::A)
  }
  #[inline]
  pub fn f(&self) -> u8 {
    self.get_r8(Reg8::F)
  }
  #[inline]
  pub fn b(&self) -> u8 {
    self.get_r8(Reg8::B)
  }
  #[inline]
  pub fn c(&self) -> u8 {
    self.get_r8(Reg8::C)
  }
  #[inline]
  pub fn d(&self) -> u8 {
    self.get_r8(Reg8::D)
  }
  #[inline]
  pub fn e(&self) -> u8 {
    self.get_r8(Reg8::E)
  }
  #[inline]
  pub fn h(&self) -> u8 {
    self.get_r8(Reg8::H)
  }
  #[inline]
  pub fn l(&self) -> u8 {
    self.get_r8(Reg8::L)
  }
  #[inline]
  pub fn imm_l(&self) -> u8 {
    self.get_r8(Reg8::ImmL)
  }
  #[inline]
  pub fn set_a(&mut self, u: u8) {
    self.set_r8(Reg8::A, u)
  }
  #[inline]
  pub fn set_f(&mut self, u: u8) {
    self.set_r8(Reg8::F, u)
  }
  #[inline]
  pub fn set_b(&mut self, u: u8) {
    self.set_r8(Reg8::B, u)
  }
  #[inline]
  pub fn set_c(&mut self, u: u8) {
    self.set_r8(Reg8::C, u)
  }
  #[inline]
  pub fn set_d(&mut self, u: u8) {
    self.set_r8(Reg8::D, u)
  }
  #[inline]
  pub fn set_e(&mut self, u: u8) {
    self.set_r8(Reg8::E, u)
  }
  #[inline]
  pub fn set_h(&mut self, u: u8) {
    self.set_r8(Reg8::H, u)
  }
  #[inline]
  pub fn set_l(&mut self, u: u8) {
    self.set_r8(Reg8::L, u)
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
        self.set_r8(r8, byte);
        let new_address = address.wrapping_add(i16::from(i) as u16);
        self.set_r16(r16, new_address);
      }
      Write(r16, r8, i) => {
        let address = self.get_r16(r16);
        let byte = self.get_r8(r8);
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
        let r = self.get_r8(r8);
        let new_r = r.wrapping_add(i as u8);
        dbg!(r, new_r);
        self.set_r8(r8, new_r);
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
        if self.is_cond(cond) {
          let pc = self.pc();
          let delta: i8 = self.imm_l() as i8;
          let new_pc = pc.wrapping_add_signed(i16::from(delta));
          self.set_pc(new_pc);
        } else {
          debug_assert_eq!(self.queue.len(), 1);
          self.queue.clear();
        }
      }
      Jp(cond) => {
        if self.is_cond(cond) {
          self.set_pc(self.get_r16(Imm));
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
      Move(left, right) => self.set_r8(left, self.get_r8(right)),
      Compare(r8) => {
        let a = self.a();
        let r = self.get_r8(r8);
        let (new_a, carry) = a.overflowing_sub(r);
        self.set_f_zero(new_a == 0);
        self.set_f_sub(true);
        self.set_f_half(new_a & 0xF > a & 0xF);
        self.set_f_carry(carry);
      }
      Add(r8, with_carry) => {
        let a = self.a();
        let rhs = self.get_r8(r8);
        let carry_in = if with_carry { self.f_carry() } else { false };
        let (new_a0, cy0) = a.overflowing_add(rhs);
        let (new_a1, cy1) = new_a0.overflowing_add(u8::from(carry_in));
        let (new_a, cy) = (new_a1, cy0 | cy1);
        self.set_a(new_a);
        self.set_f_zero(new_a == 0);
        self.set_f_sub(false);
        self.set_f_half((new_a & 0xF) < (a & 0xF) + u8::from(carry_in));
        self.set_f_carry(cy);
      }
      Sub(r8, with_carry) => {
        let a = self.a();
        let rhs = self.get_r8(r8);
        let carry_in = if with_carry { self.f_carry() } else { false };
        let (new_a0, cy0) = a.overflowing_sub(rhs);
        let (new_a1, cy1) = new_a0.overflowing_sub(u8::from(carry_in));
        let (new_a, cy) = (new_a1, cy0 | cy1);
        self.set_a(new_a);
        self.set_f_zero(new_a == 0);
        self.set_f_sub(true);
        self.set_f_half((new_a & 0xF) + u8::from(carry_in) > (a & 0xF));
        self.set_f_carry(cy);
      }
      And(r8) => {
        let a = self.a();
        let rhs = self.get_r8(r8);
        let new_a = a & rhs;
        self.set_a(new_a);
        self.set_f_zero(new_a == 0);
        self.set_f_sub(false);
        self.set_f_half(true);
        self.set_f_carry(false);
      }
      Xor(r8) => {
        let a = self.a();
        let rhs = self.get_r8(r8);
        let new_a = a ^ rhs;
        self.set_a(new_a);
        self.set_f_zero(new_a == 0);
        self.set_f_sub(false);
        self.set_f_half(false);
        self.set_f_carry(false);
      }
      Or(r8) => {
        let a = self.a();
        let rhs = self.get_r8(r8);
        let new_a = a | rhs;
        self.set_a(new_a);
        self.set_f_zero(new_a == 0);
        self.set_f_sub(false);
        self.set_f_half(false);
        self.set_f_carry(false);
      }
      MovePC(r16) => self.set_pc(self.get_r16(r16)),
      Call(cond) => {
        if self.is_cond(cond) {
          self.set_sp(self.sp().wrapping_sub(1));
          self.queue.extend(
            [
              // rustfmt don't make this a one-liner
              Write(SP, PCH, -1),
              Wr0(SP, PCL),
              MovePC(Imm),
            ]
            .into_iter(),
          );
        }
      }
      SetPC(u) => self.set_pc(u16::from(u)),
      RetIf(cond) => {
        if !self.is_cond(cond) {
          self.queue.clear()
        }
      }
      SetIE(enabled) => self.set_irq_enabled(enabled),
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
  /// 0xFF00 + reg8
  HiPg(Reg8),
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

  /// Conditional Jump to an absolute address in `Imm`
  /// * On a **failed** jump condition, flush the queue.
  Jp(Cond),

  /// Conditional return
  /// * On a **failed** condition, flush the queue.
  RetIf(Cond),

  /// Call if the condition holds.
  /// * On Condition: subtract 1 from SP then queues the rest of the call
  ///   process
  Call(Cond),

  /// (Flags)
  DecimalAdjustA,

  /// (Flags)
  ComplimentA,

  /// (Flags)
  ComplimentCarryFlag,

  /// (Flags)
  SetCarryFlag,

  /// left = right;
  Move(Reg8, Reg8),

  /// PC = r16;
  MovePC(Reg16),

  /// (Flags) void(A - r8)
  Compare(Reg8),

  /// (Z 0 H C) a += r8 ?+ carry;
  Add(Reg8, bool),

  /// (Z 1 H C) a -= r8 ?- carry;
  Sub(Reg8, bool),

  /// (Z 0 1 0) a &= r8
  And(Reg8),

  /// (Z 0 1 0) a ^= r8
  Xor(Reg8),

  /// (Z 0 0 0) a |= r8
  Or(Reg8),

  /// Sets PC to the magic value given.
  SetPC(u8),

  /// Sets IME to the magic value given.
  SetIE(bool),
}
use Action::*;
/// `Read(r8, PC, 1)`, Read PC into a Reg8, then offset PC by 1.
#[allow(bad_style)]
const fn RePC(r8: Reg8) -> Action {
  Read(r8, PC, 1)
}
/// `Read(r8, r16, 1)`
#[allow(bad_style)]
const fn Re0(r8: Reg8, r16: Reg16) -> Action {
  Read(r8, r16, 0)
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

static OP_TABLE: [&[Action]; 256] = [
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
  &[RePC(ImmL), JpRel(NC), Nop],               /* JR NC, e8 */
  &[RePC(SPL), RePC(SPH), Nop],                /* LD SP, n16 */
  &[Write(HL, A, -1), Nop],                    /* LD [HL-], A */
  &[Inc16(SP), Nop],                           /* INC SP */
  &[Re0(ImmL, HL), Inc8(ImmL), Wr0(HL, ImmL)], /* INC [HL] */
  &[Re0(ImmL, HL), Dec8(ImmL), Wr0(HL, ImmL)], /* DEC [HL] */
  &[RePC(ImmL), Wr0(HL, ImmL), Nop],           /* LD [HL], n8 */
  &[SetCarryFlag],                             /* SCF */
  &[RePC(ImmL), JpRel(Cy), Nop],               /* JR C, e8 */
  &[AddHL(SP), Nop],                           /* ADD HL, SP */
  &[Read(A, HL, -1), Nop],                     /* LD A, [HL-] */
  &[Dec16(SP), Nop],                           /* DEC SP */
  &[Inc8(A)],                                  /* INC A */
  &[Dec8(A)],                                  /* DEC A */
  &[RePC(A), Nop],                             /* LD A, n8 */
  &[ComplimentCarryFlag],                      /* CCF */
  // 0x40 series
  &[Move(B, B)],                   /* LD B, B */
  &[Move(B, C)],                   /* LD B, C */
  &[Move(B, D)],                   /* LD B, D */
  &[Move(B, E)],                   /* LD B, E */
  &[Move(B, H)],                   /* LD B, H */
  &[Move(B, L)],                   /* LD B, L */
  &[Re0(ImmL, HL), Move(B, ImmL)], /* LD B, [HL] */
  &[Move(B, A)],                   /* LD B, A */
  &[Move(C, B)],                   /* LD C, B */
  &[Move(C, C)],                   /* LD C, C */
  &[Move(C, D)],                   /* LD C, D */
  &[Move(C, E)],                   /* LD C, E */
  &[Move(C, H)],                   /* LD C, H */
  &[Move(C, L)],                   /* LD C, L */
  &[Re0(ImmL, HL), Move(C, ImmL)], /* LD C, [HL] */
  &[Move(C, A)],                   /* LD C, A */
  // 0x50 series
  &[Move(D, B)],                   /* LD D, B */
  &[Move(D, C)],                   /* LD D, C */
  &[Move(D, D)],                   /* LD D, D */
  &[Move(D, E)],                   /* LD D, E */
  &[Move(D, H)],                   /* LD D, H */
  &[Move(D, L)],                   /* LD D, L */
  &[Re0(ImmL, HL), Move(D, ImmL)], /* LD D, [HL] */
  &[Move(D, A)],                   /* LD D, A */
  &[Move(E, B)],                   /* LD E, B */
  &[Move(E, C)],                   /* LD E, C */
  &[Move(E, D)],                   /* LD E, D */
  &[Move(E, E)],                   /* LD E, E */
  &[Move(E, H)],                   /* LD E, H */
  &[Move(E, L)],                   /* LD E, L */
  &[Re0(ImmL, HL), Move(E, ImmL)], /* LD E, [HL] */
  &[Move(E, A)],                   /* LD E, A */
  // 0x60 series
  &[Move(H, B)],                   /* LD H, B */
  &[Move(H, C)],                   /* LD H, C */
  &[Move(H, D)],                   /* LD H, D */
  &[Move(H, E)],                   /* LD H, E */
  &[Move(H, H)],                   /* LD H, H */
  &[Move(H, L)],                   /* LD H, L */
  &[Re0(ImmL, HL), Move(H, ImmL)], /* LD H, [HL] */
  &[Move(H, A)],                   /* LD H, A */
  &[Move(L, B)],                   /* LD L, B */
  &[Move(L, C)],                   /* LD L, C */
  &[Move(L, D)],                   /* LD L, D */
  &[Move(L, E)],                   /* LD L, E */
  &[Move(L, H)],                   /* LD L, H */
  &[Move(L, L)],                   /* LD L, L */
  &[Re0(ImmL, HL), Move(L, ImmL)], /* LD L, [HL] */
  &[Move(L, A)],                   /* LD L, A */
  // 0x70 series
  &[Write(HL, B, 0), Nop],         /* LD [HL], B */
  &[Write(HL, C, 0), Nop],         /* LD [HL], C */
  &[Write(HL, D, 0), Nop],         /* LD [HL], D */
  &[Write(HL, E, 0), Nop],         /* LD [HL], E */
  &[Write(HL, H, 0), Nop],         /* LD [HL], H */
  &[Write(HL, L, 0), Nop],         /* LD [HL], L */
  &[Nop],                          /* HALT */
  &[Write(HL, A, 0), Nop],         /* LD [HL], A */
  &[Move(A, B)],                   /* LD A, B */
  &[Move(A, C)],                   /* LD A, C */
  &[Move(A, D)],                   /* LD A, D */
  &[Move(A, E)],                   /* LD A, E */
  &[Move(A, H)],                   /* LD A, H */
  &[Move(A, L)],                   /* LD A, L */
  &[Re0(ImmL, HL), Move(A, ImmL)], /* LD A, [HL] */
  &[Move(A, A)],                   /* LD A, A */
  // 0x80 series
  &[Add(B, false)],                   /* ADD A, B */
  &[Add(C, false)],                   /* ADD A, C */
  &[Add(D, false)],                   /* ADD A, D */
  &[Add(E, false)],                   /* ADD A, E */
  &[Add(H, false)],                   /* ADD A, H */
  &[Add(L, false)],                   /* ADD A, L */
  &[Re0(ImmL, HL), Add(ImmL, false)], /* ADD A, [HL] */
  &[Add(A, false)],                   /* ADD A, A */
  &[Add(B, true)],                    /* ADC A, B */
  &[Add(C, true)],                    /* ADC A, C */
  &[Add(D, true)],                    /* ADC A, D */
  &[Add(E, true)],                    /* ADC A, E */
  &[Add(H, true)],                    /* ADC A, H */
  &[Add(L, true)],                    /* ADC A, L */
  &[Re0(ImmL, HL), Add(ImmL, true)],  /* ADC A, [HL] */
  &[Add(A, true)],                    /* ADC A, A */
  // 0x90 series
  &[Sub(B, false)],                   /* SUB B */
  &[Sub(C, false)],                   /* SUB C */
  &[Sub(D, false)],                   /* SUB D */
  &[Sub(E, false)],                   /* SUB E */
  &[Sub(H, false)],                   /* SUB H */
  &[Sub(L, false)],                   /* SUB L */
  &[Re0(ImmL, HL), Sub(ImmL, false)], /* SUB [HL] */
  &[Sub(A, false)],                   /* SUB A */
  &[Sub(B, true)],                    /* SBC A, B */
  &[Sub(C, true)],                    /* SBC A, C */
  &[Sub(D, true)],                    /* SBC A, D */
  &[Sub(E, true)],                    /* SBC A, E */
  &[Sub(H, true)],                    /* SBC A, H */
  &[Sub(L, true)],                    /* SBC A, L */
  &[Re0(ImmL, HL), Sub(ImmL, true)],  /* SBC A, [HL] */
  &[Sub(A, true)],                    /* SBC A, A */
  // 0xA0 series
  &[And(B)],                   /* AND B */
  &[And(C)],                   /* AND C */
  &[And(D)],                   /* AND D */
  &[And(E)],                   /* AND E */
  &[And(H)],                   /* AND H */
  &[And(L)],                   /* AND L */
  &[Re0(ImmL, HL), And(ImmL)], /* AND [HL] */
  &[And(A)],                   /* AND A */
  &[Xor(B)],                   /* XOR B */
  &[Xor(C)],                   /* XOR C */
  &[Xor(D)],                   /* XOR D */
  &[Xor(E)],                   /* XOR E */
  &[Xor(H)],                   /* XOR H */
  &[Xor(L)],                   /* XOR L */
  &[Re0(ImmL, HL), Xor(ImmL)], /* XOR [HL] */
  &[Xor(A)],                   /* XOR A */
  // 0xB0 series
  &[Or(B)],                        /* OR B */
  &[Or(C)],                        /* OR C */
  &[Or(D)],                        /* OR D */
  &[Or(E)],                        /* OR E */
  &[Or(H)],                        /* OR H */
  &[Or(L)],                        /* OR L */
  &[Re0(ImmL, HL), Or(ImmL)],      /* OR [HL] */
  &[Or(A)],                        /* OR A */
  &[Compare(B)],                   /* CP B */
  &[Compare(C)],                   /* CP C */
  &[Compare(D)],                   /* CP D */
  &[Compare(E)],                   /* CP E */
  &[Compare(H)],                   /* CP H */
  &[Compare(L)],                   /* CP L */
  &[Re0(ImmL, HL), Compare(ImmL)], /* CP [HL] */
  &[Compare(A)],                   /* CP A */
  // 0xC0 series
  &[Nop, RetIf(NZ), Read(PCL, SP, 1), Read(PCH, SP, 1), Nop], /* RET NZ */
  &[Read(C, SP, 1), Read(B, SP, 1), Nop],                     /* POP BC */
  &[RePC(ImmL), RePC(ImmH), Jp(NZ), Nop],                     /* JP NZ, a16 */
  &[RePC(ImmL), RePC(ImmH), Jp(Al), Nop],                     /* JP a16 */
  &[RePC(ImmL), RePC(ImmH), Call(NZ)],                        /* CALL NZ, a16 */
  &[Dec16(SP), Write(SP, B, -1), Wr0(SP, C), Nop],            /* PUSH BC */
  &[RePC(ImmL), Add(ImmL, false)],                            /* ADD A, n8 */
  &[Dec16(SP), Write(SP, PCH, -1), Wr0(SP, PCL), SetPC(0x00)], /* RST 00H */
  &[Nop, RetIf(Ze), Read(PCL, SP, 1), Read(PCH, SP, 1), Nop], /* RET Z */
  &[Read(PCL, SP, 1), Read(PCH, SP, 1), Nop, Nop],            /* RET */
  &[RePC(ImmL), RePC(ImmH), Jp(Ze), Nop],                     /* JP Z, a16 */
  &[Nop],                                                     /* CB Prefix */
  &[RePC(ImmL), RePC(ImmH), Call(Ze)],                        /* Call Z, a16 */
  &[RePC(ImmL), RePC(ImmH), Call(Al)],                        /* CALL a16 */
  &[RePC(ImmL), Add(ImmL, true)],                             /* ADC A, n8 */
  &[Dec16(SP), Write(SP, PCH, -1), Wr0(SP, PCL), SetPC(0x08)], /* RST 08H */
  // 0xD0 series
  &[Nop, RetIf(NC), Read(PCL, SP, 1), Read(PCH, SP, 1), Nop], /* RET NC */
  &[Read(E, SP, 1), Read(D, SP, 1), Nop],                     /* POP DE */
  &[RePC(ImmL), RePC(ImmH), Jp(NC), Nop],                     /* JP NC, a16 */
  &[Nop],                                                     /* Illegal */
  &[RePC(ImmL), RePC(ImmH), Call(NC)],                        /* CALL NC, a16 */
  &[Dec16(SP), Write(SP, D, -1), Wr0(SP, E), Nop],            /* PUSH DE */
  &[RePC(ImmL), Sub(ImmL, false)],                            /* SUB n8 */
  &[Dec16(SP), Write(SP, PCH, -1), Wr0(SP, PCL), SetPC(0x10)], /* RST 10H */
  &[Nop, RetIf(Cy), Read(PCL, SP, 1), Read(PCH, SP, 1), Nop], /* RET C */
  &[Read(PCL, SP, 1), Read(PCH, SP, 1), Nop, SetIE(true)],    /* RETI */
  &[RePC(ImmL), RePC(ImmH), Jp(Cy), Nop],                     /* JP C, a16 */
  &[Nop],                                                     /* Illegal */
  &[RePC(ImmL), RePC(ImmH), Call(Cy)],                        /* CALL C, a16 */
  &[Nop],                                                     /* Illegal */
  &[RePC(ImmL), Sub(ImmL, true)],                             /* SBC A, n8 */
  &[Dec16(SP), Write(SP, PCH, -1), Wr0(SP, PCL), SetPC(0x18)], /* RST 18H */
  // 0xE0 series
  &[RePC(ImmL), Write(HiPg(ImmL), A, 0), Nop], /* LDH [a8], A */
  &[Read(L, SP, 1), Read(H, SP, 1), Nop],      /* POP HL */
  &[Write(HiPg(C), A, 0), Nop],                /* LD [C] A */
  &[Nop],                                      /* Illegal */
  &[Nop],                                      /* Illegal */
  &[Dec16(SP), Write(SP, H, -1), Wr0(SP, L), Nop], /* PUSH HL */
  &[RePC(ImmL), And(ImmL)],                    /* AND n8 */
  &[Dec16(SP), Write(SP, PCH, -1), Wr0(SP, PCL), SetPC(0x20)], /* RST 20H */
  &[Nop],                                      /* ADD SP, e8 */
  &[MovePC(HL)],                               /* JP HL */
  &[RePC(ImmL), RePC(ImmH), Wr0(Imm, A), Nop], /* LD [a16], A */
  &[Nop],                                      /* Illegal */
  &[Nop],                                      /* Illegal */
  &[Nop],                                      /* Illegal */
  &[RePC(ImmL), Xor(ImmL)],                    /* XOR n8 */
  &[Dec16(SP), Write(SP, PCH, -1), Wr0(SP, PCL), SetPC(0x28)], /* RST 28H */
  // 0xF0 series
  &[RePC(ImmL), Read(A, HiPg(ImmL), 0), Nop], /* LDH A, [a8] */
  &[Read(F, SP, 1), Read(A, SP, 1), Nop],     /* POP AF */
  &[Read(A, HiPg(C), 0), Nop],                /* LD A, [C] */
  &[SetIE(false)],                            /* DI */
  &[Nop],                                     /* Illegal */
  &[Dec16(SP), Write(SP, A, -1), Wr0(SP, F), Nop], /* PUSH AF */
  &[RePC(ImmL), Or(ImmL)],                    /* OR n8 */
  &[Dec16(SP), Write(SP, PCH, -1), Wr0(SP, PCL), SetPC(0x30)], /* RST 30H */
  &[Nop],                                     /* LD HL, SP+e8 */
  &[Move(SPH, H), Move(SPL, L)],              /* LD SP, HL */
  &[RePC(ImmL), RePC(ImmH), Re0(A, Imm), Nop], /* LD A, [a16] */
  &[SetIE(true)],                             /* EI */
  &[Nop],                                     /* Illegal */
  &[Nop],                                     /* Illegal */
  &[RePC(ImmL), Compare(ImmL)],               /* CP n8 */
  &[Dec16(SP), Write(SP, PCH, -1), Wr0(SP, PCL), SetPC(0x38)], /* RST 38H */
];

/*
prefix decoding:
all instructions are 1m for the prefix and 1m for the op itself,
unless [HL] is used in which case it's 2m to do the op itself.

low three bits:
B, C, D, E, H, L, [HL], A

upper five bits:
RLC
RRC
RL
RR
SLA
SRA
SWAP
SRL
BIT 0 through 7
RES 0 though 7
SET 0 through 7
*/
