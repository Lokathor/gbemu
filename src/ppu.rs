#[derive(Debug, Clone, Default)]
pub struct PPU {
  current_scanline: u8,
  this_line_duration: u16,
  mode: PpuMode,
}
impl PPU {
  /// scan + draw + blank time always adds up to this many
  const DOTS_HORIZONTAL: u16 = 456;
  /// current line is always in `0..=LINES_VERTICAL`
  const LINES_VERTICAL: u8 = 153;

  pub fn t_cycle(&mut self) -> PpuMode {
    self.this_line_duration += 1;
    if self.this_line_duration == Self::DOTS_HORIZONTAL {
      self.this_line_duration = 0;
      self.current_scanline += 1;
      //
      if self.current_scanline == Self::LINES_VERTICAL {
        self.current_scanline = 0;
      }
    }
    self.mode = if self.current_scanline >= 144 {
      PpuMode::VBlank
    } else if self.this_line_duration >= 200 {
      // TODO: the "200" number is a generalization and can vary from line to line
      // depending on object and window usage.
      PpuMode::HBlank
    } else if self.this_line_duration >= 80 {
      PpuMode::Draw
    } else {
      PpuMode::Search
    };
    self.mode
  }

  pub fn current_scanline(&self) -> u8 {
    self.current_scanline
  }
  pub fn mode(&self) -> PpuMode {
    self.mode
  }
}

#[derive(Debug, Clone, Copy, Default, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct OamEntry {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  pub flags: u8,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum PpuMode {
  HBlank = 0,
  VBlank = 1,
  #[default]
  Search = 2,
  Draw = 3,
}
