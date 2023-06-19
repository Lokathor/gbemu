#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::single_match)]

use bitfrob::{u8_get_bit, u8_get_value, u8_with_bit, u8_with_value};
use button_state::ButtonState;
use cpu::{CpuMode, CpuView, SM83};
use spare_parts::SpareParts;

pub mod button_state;
pub mod cpu;
pub mod mbc1;
pub mod mmio;
pub mod ppu;
pub mod spare_parts;
pub mod system;
