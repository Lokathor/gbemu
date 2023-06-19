use crate::{
  button_state::ButtonState,
  cpu::{CpuMode, MemoryBus, SM83},
  extern_parts::ExternParts,
};

pub struct System {
  cpu: SM83,
  cpu_mode: CpuMode,
  parts: ExternParts,
}
impl System {
  #[inline]
  pub fn from_cart(cart: Box<dyn MemoryBus>) -> Self {
    let mut cpu = SM83::default();
    cpu.set_pc(0x100);
    Self { cpu, cpu_mode: CpuMode::Normal, parts: ExternParts::from_cart(cart) }
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
  pub fn m_cycle(&mut self) {
    self.parts.m_cycle();
    match self.cpu_mode {
      CpuMode::Normal => self.cpu_mode = self.cpu.m_cycle(&mut self.parts),
      CpuMode::Halted => {
        // wake up only once an interrupt is ready
        if self.parts.check_pending_irqs() != 0 {
          self.cpu_mode = self.cpu.m_cycle(&mut self.parts)
        }
      }
      CpuMode::Stopped => todo!(),
    }
  }
}
