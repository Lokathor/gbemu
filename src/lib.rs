#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::result_unit_err)]

use bitfrob::{u8_with_bit, u8_with_value};
use cpu::MemoryBus;

pub mod cpu;
pub mod mbc1;

const ROM_BANK_SIZE: usize = 16 * 1024;
const ERAM_BANK_SIZE: usize = 8 * 1024;
