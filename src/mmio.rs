use bitfrob::{u8_get_bit, u8_with_bit, u8_with_value};

use crate::{button_state::ButtonState, ppu::PpuMode, spare_parts::IrqTy};

#[derive(Debug, Clone)]
pub struct MMIO {
  bytes: [u8; 256],
  button_state: ButtonState,
  serial_log: Option<Vec<u8>>,
  timer_sub_ticks: u16,
}
impl Default for MMIO {
  #[inline]
  fn default() -> Self {
    let mut out = Self {
      bytes: [0_u8; 256],
      button_state: Default::default(),
      serial_log: None,
      timer_sub_ticks: TimerControl(0).sub_ticks(),
    };
    out.write(MMIO::JOYP as _, 0_u8);
    out
  }
}
impl MMIO {
  pub const JOYP: usize = 0x00;
  pub const SB: usize = 0x01;
  pub const SC: usize = 0x02;
  pub const TIMA: usize = 0x05;
  pub const TMA: usize = 0x06;
  pub const TAC: usize = 0x07;
  pub const IF: usize = 0x0F;
  pub const STAT: usize = 0x41;
  pub const LY: usize = 0x44;
  pub const LYC: usize = 0x45;

  #[inline]
  pub fn read(&self, index: u8) -> u8 {
    self.bytes[usize::from(index)]
  }
  #[inline]
  pub fn write(&mut self, index: u8, byte: u8) {
    match usize::from(index) {
      MMIO::JOYP => {
        let action = !u8_get_bit(5, byte);
        let direction = !u8_get_bit(4, byte);
        self.bytes[MMIO::JOYP] = self.button_state.to_joyp(action, direction);
      }
      MMIO::SC => {
        let sc = SerialControl(byte);
        if sc.transfer() {
          let sb = self.sb();
          if let Some(log) = &mut self.serial_log {
            log.push(sb)
          }
        }
        self.bytes[Self::SC] = byte;
      }
      MMIO::STAT => {
        // bits 0..=2 are read-only to the CPU.
        let old_stat = self.bytes[Self::STAT];
        let fixed_stat = u8_with_value(0, 2, byte, old_stat);
        self.bytes[MMIO::STAT] = fixed_stat;
      }
      other => self.bytes[other] = byte,
    }
  }
  #[inline]
  pub fn set_serial_logging(&mut self, log: bool) {
    if log {
      self.serial_log = Some(vec![]);
    } else {
      self.serial_log = None;
    }
  }
  #[inline]
  pub fn serial_log(&self) -> &[u8] {
    self.serial_log.as_deref().unwrap_or_default()
  }

  #[inline]
  pub fn set_button_state(&mut self, button_state: ButtonState) {
    let old_joyp = self.joyp() & 0b1111;
    self.button_state = button_state;
    // flag an interrupt if any of the low 4 bits were *not* clear and now they
    // *are* clear.
    let new_joyp = self.joyp() & 0b1111;
    let diff_joyp = (old_joyp ^ new_joyp) & 0b1111;
    if diff_joyp != 0 && new_joyp != 0 {
      self.flag_interrupt(IrqTy::Joypad);
    }
  }
  #[inline]
  pub fn set_ly(&mut self, byte: u8) {
    // flag interrupt when we first get to line 144
    if byte == 144 && self.bytes[Self::LY] < 144 {
      self.flag_interrupt(IrqTy::VBlank);
    }
    // always keep the lyc match bit accurate.
    self.bytes[Self::STAT] = u8_with_bit(2, self.bytes[Self::STAT], byte == self.lyc());
    self.bytes[Self::LY] = byte;
  }
  #[inline]
  pub fn set_ppu_mode(&mut self, mode: PpuMode) {
    self.bytes[Self::STAT] = u8_with_value(0, 1, self.bytes[Self::STAT], mode as u8);
    // TODO: trigger STAT interrupts... somehow?
  }

  #[inline]
  pub fn m_cycle(&mut self) {
    let tac = self.tac();
    // if timer enabled
    if tac.enabled() {
      debug_assert_ne!(self.timer_sub_ticks, 0);
      self.timer_sub_ticks -= 1;
      if self.timer_sub_ticks == 0 {
        self.timer_sub_ticks = tac.sub_ticks();
        let (new, overflow) = self.tima().overflowing_add(1);
        if overflow {
          self.bytes[Self::TIMA] = self.tma();
          self.flag_interrupt(IrqTy::Timer);
        } else {
          self.bytes[Self::TIMA] = new;
        }
      }
    }
  }

  #[inline]
  pub fn joyp(&self) -> u8 {
    self.read(Self::JOYP as u8)
  }
  #[inline]
  pub fn sb(&self) -> u8 {
    self.read(Self::SB as u8)
  }
  #[inline]
  pub fn sc(&self) -> u8 {
    self.read(Self::SC as u8)
  }
  #[inline]
  pub fn tima(&self) -> u8 {
    self.read(Self::TIMA as u8)
  }
  #[inline]
  pub fn tma(&self) -> u8 {
    self.read(Self::TMA as u8)
  }
  #[inline]
  pub fn tac(&self) -> TimerControl {
    TimerControl(self.read(Self::TAC as u8))
  }
  #[inline]
  pub fn if_(&self) -> u8 {
    self.read(Self::IF as u8)
  }
  #[inline]
  pub fn lyc(&self) -> u8 {
    self.read(Self::LYC as u8)
  }

  #[inline]
  pub fn flag_interrupt(&mut self, ty: IrqTy) {
    self.bytes[Self::IF] |= 1 << (ty as u8);
  }
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct SerialControl(u8);
impl SerialControl {
  #[inline]
  pub fn transfer(self) -> bool {
    u8_get_bit(7, self.0)
  }
  #[inline]
  pub fn fast(self) -> bool {
    u8_get_bit(1, self.0)
  }
  #[inline]
  pub fn internal(self) -> bool {
    u8_get_bit(0, self.0)
  }
}
impl core::fmt::Debug for SerialControl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SerialControl")
      .field("transfer", &self.transfer())
      .field("fast", &self.fast())
      .field("internal", &self.internal())
      .finish()
  }
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct TimerControl(u8);
impl TimerControl {
  #[inline]
  pub fn enabled(self) -> bool {
    u8_get_bit(2, self.0)
  }
  /// The number of M-cycle sub-ticks of the timer before the main value ticks.
  #[inline]
  pub fn sub_ticks(self) -> u16 {
    match self.0 & 0b11 {
      0 => 256,
      1 => 4,
      2 => 16,
      3 => 64,
      _ => unreachable!(),
    }
  }
}
impl core::fmt::Debug for TimerControl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("TimerControl")
      .field("enabled", &self.enabled())
      .field("sub_ticks", &self.sub_ticks())
      .finish()
  }
}
