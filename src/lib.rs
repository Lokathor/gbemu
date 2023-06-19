#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::single_match)]

use bitfrob::{u8_get_bit, u8_get_value, u8_with_bit, u8_with_value};
use button_state::ButtonState;
use cpu::{CpuMode, MemoryBus, SM83};
use extern_parts::ExternParts;

pub mod button_state;
pub mod cpu;
pub mod extern_parts;
pub mod mbc1;
pub mod mmio;
pub mod system;

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
pub enum IrqTy {
  VBlank = 0,
  LCDSTAT = 1,
  Timer = 2,
  Serial = 3,
  Joypad = 4,
}
