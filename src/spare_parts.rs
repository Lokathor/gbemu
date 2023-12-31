use core::ops::Range;

use crate::{
  button_state::ButtonState,
  cpu::CpuView,
  mmio::{SerialControl, TimerControl, MMIO},
  ppu::OamEntry,
};

pub const ROM_BANK_SIZE: usize = 16 * 1024;
pub const SRAM_BANK_SIZE: usize = 8 * 1024;
pub const VRAM_BANK_SIZE: usize = 8 * 1024;
pub const WRAM_BANK_SIZE: usize = 4 * 1024;

/// Parts of the system that are external to the CPU.
pub struct SpareParts {
  /// Any ROM and SRAM accesses pass through to here.
  cart: Box<dyn CpuView>,
  /// Video RAM (one slot visible)
  vram: Box<[[u8; VRAM_BANK_SIZE]; 2]>,
  vram_bank: usize,
  /// Work RAM (always zero, plus one other slot visible)
  wram: Box<[[u8; WRAM_BANK_SIZE]; 8]>,
  wram_bank: usize,
  /// Object Attributes
  oam: [OamEntry; 40],
  /// IO controls and high page ram
  mmio: MMIO,
}
impl SpareParts {
  pub fn from_cart(cart: Box<dyn CpuView>) -> Self {
    Self {
      cart,
      vram: bytemuck::allocation::zeroed_box(),
      vram_bank: 0,
      wram: bytemuck::allocation::zeroed_box(),
      wram_bank: 1,
      oam: [Default::default(); 40],
      mmio: Default::default(),
    }
  }

  #[inline]
  pub fn set_button_state(&mut self, button_state: ButtonState) {
    self.mmio.set_button_state(button_state);
  }
  #[inline]
  pub fn set_serial_logging(&mut self, log: bool) {
    self.mmio.set_serial_logging(log);
  }
  #[inline]
  pub fn serial_log(&self) -> &[u8] {
    self.mmio.serial_log()
  }

  #[inline]
  pub fn m_cycle(&mut self) {
    self.mmio.m_cycle();
  }
  #[inline]
  pub fn mmio(&self) -> &MMIO {
    &self.mmio
  }
  #[inline]
  pub fn mmio_mut(&mut self) -> &mut MMIO {
    &mut self.mmio
  }
}
impl CpuView for SpareParts {
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
      0xFF00..=0xFFFF => self.mmio.read(address as u8),
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
      0xFF00..=0xFFFF => self.mmio.write(address as u8, byte),
      // "illegal" location writes will just do nothing.
      _ => (),
    }
  }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum IrqTy {
  VBlank = 0,
  LCDSTAT = 1,
  Timer = 2,
  Serial = 3,
  Joypad = 4,
}
