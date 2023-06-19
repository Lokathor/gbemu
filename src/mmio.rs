use bitfrob::u8_get_bit;

use crate::{button_state::ButtonState, IrqTy};

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
      timer_sub_ticks: 0,
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

  pub fn read(&self, index: u8) -> u8 {
    self.bytes[usize::from(index)]
  }
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
      MMIO::TAC => {
        self.timer_sub_ticks = self.tac().sub_ticks();
        self.bytes[Self::TAC] = byte;
      }
      other => self.bytes[other] = byte,
    }
  }

  #[inline]
  pub fn set_button_state(&mut self, button_state: ButtonState) {
    self.button_state = button_state;
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
          self.set_tima(self.tma());
          self.flag_interrupt(IrqTy::Timer);
        } else {
          self.set_tima(new);
        }
      }
    }
  }

  pub fn joyp(&self) -> u8 {
    self.read(Self::JOYP as u8)
  }
  pub fn sb(&self) -> u8 {
    self.read(Self::SB as u8)
  }
  pub fn sc(&self) -> u8 {
    self.read(Self::SC as u8)
  }
  pub fn tima(&self) -> u8 {
    self.read(Self::TIMA as u8)
  }
  pub fn tma(&self) -> u8 {
    self.read(Self::TMA as u8)
  }
  pub fn tac(&self) -> TimerControl {
    TimerControl(self.read(Self::TAC as u8))
  }
  pub fn if_(&self) -> u8 {
    self.read(Self::IF as u8)
  }

  pub fn set_tima(&mut self, byte: u8) {
    self.write(Self::TIMA as u8, byte)
  }
  pub fn set_sc(&mut self, sc: SerialControl) {
    self.write(Self::SC as u8, sc.0)
  }

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