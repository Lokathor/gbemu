#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::single_match)]

use bitfrob::{u8_get_bit, u8_get_value, u8_with_bit, u8_with_value};
use cpu::MemoryBus;

pub mod cpu;
pub mod mbc1;

const ROM_BANK_SIZE: usize = 16 * 1024;
const SRAM_BANK_SIZE: usize = 8 * 1024;
const VRAM_BANK_SIZE: usize = 8 * 1024;
const WRAM_BANK_SIZE: usize = 4 * 1024;

#[derive(Debug, Clone, Copy, Default, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct OamEntry {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  pub flags: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Irq {
  VBlank = 0,
  LCDSTAT = 1,
  Timer = 2,
  Serial = 3,
  Joypad = 4,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct MMIO([u8; 256]);
impl Default for MMIO {
  #[inline]
  fn default() -> Self {
    Self([0_u8; 256])
  }
}
impl MMIO {
  pub const SB: usize = 0x01;
  pub fn sb(&self) -> u8 {
    self.0[Self::SB]
  }
  pub fn set_sb(&mut self, sb: u8) {
    self.0[Self::SB] = sb;
  }

  pub const SC: usize = 0x02;
  pub fn sc(&self) -> u8 {
    self.0[Self::SC]
  }
  pub fn set_sc(&mut self, sc: u8) {
    self.0[Self::SC] = sc;
  }

  pub const TIMA: usize = 0x05;
  /// timer counter
  pub fn tima(&self) -> u8 {
    self.0[Self::TIMA]
  }
  pub fn set_tima(&mut self, tima: u8) {
    self.0[Self::TIMA] = tima;
  }

  pub const TMA: usize = 0x06;
  /// timer reload value
  pub fn tma(&self) -> u8 {
    self.0[Self::TMA]
  }
  pub fn set_tma(&mut self, tma: u8) {
    self.0[Self::TMA] = tma;
  }

  pub const TAC: usize = 0x07;
  pub fn tac(&self) -> TimerControl {
    TimerControl(self.0[Self::TAC])
  }
  pub fn set_tac(&mut self, tac: TimerControl) {
    self.0[Self::TAC] = tac.0;
  }

  pub const IF: usize = 0x0F;
  pub fn flag_interrupt(&mut self, irq: Irq) {
    self.0[Self::IF] |= (1 << (irq as u8));
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

/// Parts of the system that are external to the CPU.
pub struct ExternParts {
  /// Any ROM and SRAM accesses pass through to here.
  pub cart: Box<dyn MemoryBus>,
  /// Video RAM (one slot visible)
  pub vram: Box<[[u8; VRAM_BANK_SIZE]; 2]>,
  pub vram_bank: usize,
  /// Work RAM (always zero, plus one other slot visible)
  pub wram: Box<[[u8; WRAM_BANK_SIZE]; 8]>,
  pub wram_bank: usize,
  /// Object Attributes
  pub oam: [OamEntry; 40],
  /// IO controls and high page ram
  pub mmio: MMIO,
  /// Log of serial outputs
  pub serial_log: Option<Vec<u8>>,
  pub timer_sub_ticks: u16,
}
impl ExternParts {
  pub fn from_cart(cart: Box<dyn MemoryBus>) -> Self {
    Self {
      cart,
      vram: bytemuck::allocation::zeroed_box(),
      vram_bank: 0,
      wram: bytemuck::allocation::zeroed_box(),
      wram_bank: 1,
      oam: [Default::default(); 40],
      mmio: Default::default(),
      serial_log: None,
      timer_sub_ticks: 0,
    }
  }

  pub fn m_cycle(&mut self) {
    let tac = self.mmio.tac();
    // if timer enabled
    if tac.enabled() {
      debug_assert_ne!(self.timer_sub_ticks, 0);
      self.timer_sub_ticks -= 1;
      if self.timer_sub_ticks == 0 {
        self.timer_sub_ticks = tac.sub_ticks();
        let (new, overflow) = self.mmio.tima().overflowing_add(1);
        if overflow {
          self.mmio.set_tima(self.mmio.tma());
          self.mmio.flag_interrupt(Irq::Timer);
        } else {
          self.mmio.set_tima(new);
        }
      }
    }
  }
}
impl MemoryBus for ExternParts {
  #[inline]
  #[must_use]
  fn read(&self, address: u16) -> u8 {
    // TODO: simulate some regions being locked out when DMA is active.
    match address {
      0x0000..=0x7FFF => self.cart.read(address),
      0x8000..=0x9FFF => {
        let address = usize::from(address - 0x8000);
        self.vram[self.vram_bank][address]
      }
      0xA000..=0xBFFF => self.cart.read(address),
      0xC000..=0xCFFF => {
        let address = usize::from(address - 0xC000);
        self.wram[0][address]
      }
      0xD000..=0xDFFF => {
        let address = usize::from(address - 0xD000);
        self.wram[self.wram_bank][address]
      }
      0xFE00..=0xFE9F => {
        let address = usize::from(address - 0xE000);
        bytemuck::bytes_of(&self.oam)[address]
      }
      0xFF00..=0xFFFF => {
        let address = usize::from(address - 0xFF00);
        self.mmio.0[address]
      }
      // "illegal" location reads just see 0xFF
      _ => 0xFF,
    }
  }

  #[inline]
  fn write(&mut self, address: u16, byte: u8) {
    // TODO: simulate some regions being locked out when DMA is active.
    match address {
      // rom
      0x0000..=0x7FFF => self.cart.write(address, byte),
      // vram
      0x8000..=0x9FFF => {
        let address = usize::from(address - 0x8000);
        self.vram[self.vram_bank][address] = byte;
      }
      // sram
      0xA000..=0xBFFF => self.cart.write(address, byte),
      // wram
      0xC000..=0xCFFF => {
        let address = usize::from(address - 0xC000);
        self.wram[0][address] = byte;
      }
      0xD000..=0xDFFF => {
        let address = usize::from(address - 0xD000);
        self.wram[self.wram_bank][address] = byte;
      }
      // oam
      0xFE00..=0xFE9F => {
        let address = usize::from(address - 0xE000);
        bytemuck::bytes_of_mut(&mut self.oam)[address] = byte;
      }
      // mmio controls and hram
      0xFF00..=0xFFFF => {
        let address = usize::from(address - 0xFF00);
        if let Some(serial_log) = &mut self.serial_log {
          // When we're logging, every command to transfer also pushes SB into the log.
          if address == MMIO::SC && SerialControl(byte).transfer() {
            serial_log.push(self.mmio.sb());
          }
        }
        match address {
          MMIO::TIMA => {
            //
            println!("TIMA(ctr) = {byte}");
          }
          MMIO::TMA => {
            //
            println!("TMA(mod) = {byte}");
          }
          MMIO::TAC => {
            let tac = TimerControl(byte);
            println!("TAC = {tac:?}");
            if tac.enabled() {
              self.timer_sub_ticks = tac.sub_ticks();
            }
          }
          _ => (),
        }
        self.mmio.0[address] = byte;
      }
      // "illegal" location writes will just do nothing.
      _ => (),
    }
  }
}
