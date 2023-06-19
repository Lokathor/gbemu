use core::num::Wrapping;

use crate::{
  button_state::ButtonState,
  cpu::{CpuMode, CpuView, SM83},
  ppu::PPU,
  spare_parts::SpareParts,
};

pub struct System {
  cpu: SM83,
  cpu_mode: CpuMode,
  ppu: PPU,
  parts: SpareParts,
  t_clock: usize,
}
impl System {
  #[inline]
  pub fn from_cart(cart: Box<dyn CpuView>) -> Self {
    let mut cpu = SM83::default();
    cpu.set_pc(0x100);
    Self {
      cpu,
      cpu_mode: CpuMode::Normal,
      ppu: PPU::default(),
      parts: SpareParts::from_cart(cart),
      t_clock: 0,
    }
  }

  #[inline]
  pub fn set_serial_logging(&mut self, log: bool) {
    self.parts.set_serial_logging(log);
  }
  #[inline]
  pub fn serial_log(&self) -> &[u8] {
    self.parts.serial_log()
  }

  #[inline]
  pub fn set_button_state(&mut self, button_state: ButtonState) {
    self.parts.set_button_state(button_state);
  }

  #[inline]
  pub fn t_cycle(&mut self) {
    self.t_clock = self.t_clock.wrapping_add(1);
    self.ppu.t_cycle();
    //
    if self.t_clock & 0b11 == 0 {
      self.parts.mmio_mut().set_ly(self.ppu.current_scanline());
      self.parts.m_cycle();
      match self.cpu_mode {
        CpuMode::Normal => self.cpu_mode = self.cpu.m_cycle(&mut self.parts),
        CpuMode::Halted => {
          // wake up only once an interrupt is ready
          if self.parts.check_pending_irqs() != 0 {
            self.cpu_mode = self.cpu.m_cycle(&mut self.parts)
          }
        }
        CpuMode::Stopped => {
          // wake when any of the low 4 bits of `JOYP` become zero, regardless
          // of interrupt settings.
          if self.parts.mmio().joyp() & 0b1111 != 0 {
            self.cpu_mode = self.cpu.m_cycle(&mut self.parts)
          }
        }
      }
    }
  }
  #[inline]
  pub fn m_cycle(&mut self) {
    self.t_cycle();
    self.t_cycle();
    self.t_cycle();
    self.t_cycle();
  }
}
